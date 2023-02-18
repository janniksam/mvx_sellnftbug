multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait MarketplaceModule: ContractBase {
    #[endpoint(marketplaceBuy)]
    #[payable("*")]
    fn marketplace_buy(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().egld_or_single_esdt();

        //
        // commenting the following sell_nft-call out gives a SUCCESSFUL test,
        // leaving this line here, will result the test to FAIL, even though this method should NEVER be called.
        //
        // even worse, if you just add...
        //
        //      require!(false, "test");
        //
        // ... here, the test will also be SUCCESSFUL
        //

        // require!(false, "test");
        self.send().sell_nft(
            &payment.token_identifier.clone().unwrap_esdt(),
            0,
            &BigUint::from(1u64),
            &caller,
            &payment.token_identifier,
            1, // replacing thís with 0 is also making the test run SUCCESSFULLY
            &BigUint::zero(),
        );
    }
}
