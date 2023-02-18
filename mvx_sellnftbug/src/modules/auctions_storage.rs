use crate::models::auction::Auction;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AuctionsStorageModule: ContractBase {
    #[view(get_highest_auction_id)]
    #[storage_mapper("auc_highest_auction_id")]
    fn highest_auction_id(&self) -> SingleValueMapper<u64>;

    #[view(get_auctions)]
    #[storage_mapper("auc_auctions")]
    fn auctions(&self, auction_id: u64) -> SingleValueMapper<Auction<Self::Api>>;

    #[view(get_all_auctions)]
    #[storage_mapper("auc_all_auctions")]
    fn all_auctions(&self) -> UnorderedSetMapper<u64>;

    #[view(get_active_auctions)]
    #[storage_mapper("auc_active_auctions")]
    fn active_auctions(&self) -> UnorderedSetMapper<u64>;

    #[view(get_my_active_auctions)]
    #[storage_mapper("auc_my_active_auctions")]
    fn my_active_auctions(&self, creator: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(get_my_winning_bids)]
    #[storage_mapper("auc_my_winning_bids")]
    fn my_winning_bids(&self, better: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(get_highest_bid_auction)]
    #[storage_mapper("auc_highest_bid_auction")]
    fn highest_bid_auction(&self, auction_id: u64) -> SingleValueMapper<BigUint>;

    #[view(get_highest_bidder_auction)]
    #[storage_mapper("auc_highest_bidder_auction")]
    fn highest_bidder_auction(&self, auction_id: u64) -> SingleValueMapper<ManagedAddress>;
}
