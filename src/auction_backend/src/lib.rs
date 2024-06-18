use ic_cdk::export::candid::{candid_method, Principal};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};

mod auction;
mod state;
mod types;

use auction::*;
use state::State;
use types::{Item, Bid};

// Initialize the canister.
#[init]
#[candid_method(init)]
fn init() {
    state::initialize();
}

#[pre_upgrade]
fn pre_upgrade() {
    state::pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    state::post_upgrade();
}

// List an item.
#[update(name = "listItem")]
#[candid_method(update, rename = "listItem")]
fn list_item(title: String, description: String) -> u64 {
    list_new_item(title, description)
}

// Bid on an item.
#[update]
#[candid_method(update)]
fn bid(item_id: u64, amount: u64) {
    place_bid(item_id, amount);
}

// Update an item.
#[update(name = "updateItem")]
#[candid_method(update, rename = "updateItem")]
fn update_item(item_id: u64, title: String, description: String) {
    modify_item(item_id, title, description);
}

// Stop an item listing.
#[update(name = "stopListing")]
#[candid_method(update, rename = "stopListing")]
fn stop_listing(item_id: u64) {
    stop_item_listing(item_id);
}

// Query functions.
#[query]
#[candid_method(query)]
fn get_item(item_id: u64) -> Option<Item> {
    fetch_item(item_id)
}

#[query(name = "getItems")]
#[candid_method(query, rename = "getItems")]
fn get_items() -> Vec<Item> {
    fetch_items()
}

#[query(name = "getItemCount")]
#[candid_method(query, rename = "getItemCount")]
fn get_item_count() -> usize {
    fetch_item_count()
}

#[query(name = "getHighestBid")]
#[candid_method(query, rename = "getHighestBid")]
fn get_highest_bid(item_id: u64) -> Option<Bid> {
    fetch_highest_bid(item_id)
}

#[query(name = "getItemStats")]
#[candid_method(query, rename = "getItemStats")]
fn get_item_stats() -> (Option<Item>, Option<Item>) {
    fetch_item_stats()
}
