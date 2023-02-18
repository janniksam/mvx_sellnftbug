use crate::models::auction::Auction;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const MINIMUM_CHARITY_FEE: u32 = 10; // minimum is 1%
pub const MAXIMUM_CHARITY_FEE: u32 = 500; // minimum is 50%
pub const ONE_WEEK: u32 = 604800;
pub const FOUR_WEEKS: u32 = 2419200;

#[multiversx_sc::module]
pub trait AuctionsModule:
    ContractBase + crate::modules::auctions_storage::AuctionsStorageModule
{
    /*
     * create a new auction
     */
    #[endpoint(createAuction)]
    #[payable("*")]
    fn create_auction(
        &self,
        #[payment_token] token: EgldOrEsdtTokenIdentifier,
        #[payment_amount] amount: BigUint,
        #[payment_nonce] nonce: u64,
        auction_start: u64,
        auction_end: u64,
        minimum_bid: BigUint,
        charity_fees_per_thousand: u32,
    ) {
        require!(
            nonce > 0 && amount == 1 && token.is_esdt(),
            "needs to be a single nft/sft"
        );

        let now = self.blockchain().get_block_timestamp();
        require!(auction_start > now, "auction start is in the past");

        let until_start = auction_start - now;
        require!(
            until_start <= FOUR_WEEKS as u64,
            "auction needs to start in 4 weeks at most"
        );

        require!(
            auction_start < auction_end,
            "auction_end needs to be higher than auction_start"
        );

        let duration = auction_end - auction_start;
        require!(
            duration <= ONE_WEEK as u64,
            "auction duration cannot exceed 1 week"
        );

        require!(
            charity_fees_per_thousand >= MINIMUM_CHARITY_FEE,
            "at least 1 percent needs to go to charity"
        );

        require!(
            charity_fees_per_thousand <= MAXIMUM_CHARITY_FEE,
            "at most 50 percent needs to go to charity"
        );

        let caller = self.blockchain().get_caller();

        let next_id = self.highest_auction_id().update(|val| {
            *val += 1;
            *val
        });

        let inserted = self.all_auctions().insert(next_id);
        require!(inserted, "id already used"); // should NEVER happen

        self.active_auctions().insert(next_id);
        self.my_active_auctions(&caller).insert(next_id);
        let auction = Auction {
            auction_id: next_id,
            creator: caller,
            auction_start: auction_start,
            auction_end: auction_end,
            token_identifier: token,
            nonce: nonce,
            minimum_bid: minimum_bid,
            charity_fees_per_thousand: charity_fees_per_thousand,
        };
        self.auctions(next_id).set(auction);
    }

    #[endpoint(cancelAuction)]
    fn cancel_auction(&self, auction_id: u64) {
        require!(!self.auctions(auction_id).is_empty(), "auction not found",);

        let auction = self.auctions(auction_id).get();

        let caller = self.blockchain().get_caller();
        require!(
            auction.creator == caller,
            "you are not the creator of this auction"
        );

        require!(
            self.highest_bidder_auction(auction_id).is_empty(),
            "auction has already a bidder"
        );

        self.delete_auction(&auction);
        self.send().direct_esdt(
            &auction.creator,
            &auction.token_identifier.unwrap_esdt(),
            auction.nonce,
            &BigUint::from(1u64),
        );
    }

    #[endpoint(auctionBid)]
    #[payable("EGLD")]
    fn auction_bid(&self, auction_id: u64) {
        require!(!self.auctions(auction_id).is_empty(), "auction not found",);

        let auction = self.auctions(auction_id).get();
        let now = self.blockchain().get_block_timestamp();

        require!(auction.auction_start <= now, "auction has yet to start");

        require!(auction.auction_end >= now, "auction has ended");

        let bid = self.call_value().egld_value();
        require!(bid > 0, "nothing was bid");

        require!(
            auction.minimum_bid == 0 || bid >= auction.minimum_bid,
            "bid less than minimum"
        );

        let old_highest_bid = self.highest_bid_auction(auction_id).get();
        require!(bid > old_highest_bid, "bid less than highest bid");

        if old_highest_bid > 0 {
            let old_highest_bidder = self.highest_bidder_auction(auction_id).get();
            self.my_winning_bids(&old_highest_bidder)
                .swap_remove(&auction_id);
            self.send()
                .direct_egld(&old_highest_bidder, &old_highest_bid);
        }

        let caller = self.blockchain().get_caller();
        self.my_winning_bids(&caller).insert(auction_id);
        self.highest_bidder_auction(auction_id).set(&caller);
        self.highest_bid_auction(auction_id).set(bid);
    }

    #[endpoint(auctionClaim)]
    fn auction_claim(&self, auction_id: u64) {
        require!(!self.auctions(auction_id).is_empty(), "auction not found",);

        let auction = self.auctions(auction_id).get();
        let now = self.blockchain().get_block_timestamp();
        require!(auction.auction_end < now, "auction has not ended");

        let caller = self.blockchain().get_caller();
        let winner = self.highest_bidder_auction(auction_id).get();
        require!(caller == winner, "you are not the highest bidder");

        let winner_bid_fees_deducted = self.take_fee(&auction);

        self.delete_auction(&auction);
        self.my_winning_bids(&caller)
            .swap_remove(&auction.auction_id);

        let left_after_royalties = self.send().sell_nft(
            &auction.token_identifier.unwrap_esdt(),
            auction.nonce,
            &BigUint::from(1u64),
            &winner,
            &EgldOrEsdtTokenIdentifier::egld(),
            0,
            &winner_bid_fees_deducted,
        );

        self.send()
            .direct_egld(&auction.creator, &left_after_royalties);
    }

    //
    // deducts the configurated fees from the auctions bid
    //
    fn take_fee(&self, auction: &Auction<Self::Api>) -> BigUint {
        let bid = self.highest_bid_auction(auction.auction_id).get();
        let deducted_fee = &bid * auction.charity_fees_per_thousand / 1000u32;
        if deducted_fee == 0 {
            return bid;
        }

        return bid - deducted_fee;
    }

    fn delete_auction(&self, auction: &Auction<Self::Api>) {
        self.active_auctions().swap_remove(&auction.auction_id);
        self.my_active_auctions(&auction.creator)
            .swap_remove(&auction.auction_id);
        self.all_auctions().swap_remove(&auction.auction_id);
        self.auctions(auction.auction_id).clear();
        self.highest_bid_auction(auction.auction_id).clear();
        self.highest_bidder_auction(auction.auction_id).clear();
    }
}
