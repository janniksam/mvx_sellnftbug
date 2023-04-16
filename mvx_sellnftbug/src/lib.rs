#![no_std]

multiversx_sc::imports!();

mod proxy;

#[multiversx_sc::contract]
pub trait SellnftbugSC: ContractBase {
    #[view(get_destination_contract_address)]
    #[storage_mapper("destination_contract_address")]
    fn destination_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[init]
    fn init(&self, address: ManagedAddress) {
        self.destination_contract_address().set(address);
    }

    //
    //
    //
    //
    //
    // the calling side:
    //
    //
    //
    //
    //

    #[endpoint(reproduce)]
    #[payable("EGLD")]
    fn reproduce(&self) {
        let egld_amount = self.call_value().egld_value();

        self.foo_proxy(self.destination_contract_address().get())
            .foo()
            .with_egld_transfer(egld_amount)
            .with_gas_limit(self.blockchain().get_gas_left() - 10_000_000)
            .async_call()
            .with_callback(self.callbacks().foo_callback())
            .call_and_exit();
    }

    #[callback]
    fn foo_callback(&self, #[call_result] result: ManagedAsyncCallResult<()>) {
        match result {
            ManagedAsyncCallResult::Ok(_value) => {
                // fails
                let _test = self.call_value().single_esdt();
            }
            ManagedAsyncCallResult::Err(_err) => {}
        }
    }

    //
    //
    //
    //
    //
    // the called side:
    //
    //
    //
    //
    //

    #[view(get_payment)]
    #[storage_mapper("payment")]
    fn payment(&self) -> SingleValueMapper<EsdtTokenPayment>;

    #[endpoint(depositEsdt)]
    #[payable("*")]
    fn deposit_esdt(&self) {
        self.payment().set(self.call_value().single_esdt());
    }

    #[endpoint(foo)]
    #[payable("EGLD")]
    fn foo(&self) {
        let payment = self.payment().get();
        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }

    #[proxy]
    fn foo_proxy(&self, sc_address: ManagedAddress) -> crate::proxy::Proxy<Self::Api>;
}
