#![no_std]

multiversx_sc::imports!();

// A contract that manages and fulfills limit orders for users
#[multiversx_sc::contract]
pub trait SellnftbugSC: ContractBase {
    #[init]
    fn init(&self) {}

    /*
     * create a new auction
     */
    #[endpoint(reproduce)]
    #[payable("*")]
    fn reproduce(&self) {
        self.verify_balance();
        self.send().direct_egld(
            &self.blockchain().get_caller(),
            &self.call_value().egld_value(),
        )
    }

    #[view(verify_balance)]
    fn verify_balance(&self) {
        // does NOT work
        let _balance_should_have = self.call_value().egld_value() + BigUint::from(1u64);

        // works
        // let _balance_should_have = BigUint::from(1u64) + self.call_value().egld_value();
    }
}
