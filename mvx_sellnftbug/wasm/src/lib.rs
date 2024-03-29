// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           13
// Async Callback (empty):               1
// Total number of exported functions:  15

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    sellnftbug
    (
        createAuction
        cancelAuction
        auctionBid
        auctionClaim
        get_highest_auction_id
        get_auctions
        get_all_auctions
        get_active_auctions
        get_my_active_auctions
        get_my_winning_bids
        get_highest_bid_auction
        get_highest_bidder_auction
        marketplaceBuy
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
