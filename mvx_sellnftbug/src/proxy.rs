multiversx_sc::imports!();

/// this proxy is being used to wrap / unwrap eGLD
#[multiversx_sc::proxy]
pub trait FooProxy {
    #[payable("EGLD")]
    #[endpoint(foo)]
    fn foo(&self);
}
