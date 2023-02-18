use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, EgldOrEsdtTokenIdentifier, ManagedAddress},
};

multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, ManagedVecItem)]
pub struct Auction<M: ManagedTypeApi> {
    pub auction_id: u64,
    pub creator: ManagedAddress<M>,
    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
    pub auction_start: u64,
    pub auction_end: u64,
    pub minimum_bid: BigUint<M>,
    pub charity_fees_per_thousand: u32,
}
