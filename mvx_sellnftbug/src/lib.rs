#![no_std]

multiversx_sc::imports!();

mod models;
mod modules;

// A contract that manages and fulfills limit orders for users
#[multiversx_sc::contract]
pub trait SellnftbugSC:
    ContractBase
    + modules::auctions::AuctionsModule
    + modules::auctions_storage::AuctionsStorageModule
    + modules::marketplace::MarketplaceModule
{
    #[init]
    fn init(&self) {}
}
