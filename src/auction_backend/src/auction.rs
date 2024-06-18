use ic_cdk::caller;
use crate::state::{with_state, with_state_mut};
use crate::types::{Item, Bid};

pub fn list_new_item(title: String, description: String) -> u64 {
    with_state_mut(|state| {
        let item_id = state.items.len() as u64;
        let item = Item {
            id: item_id,
            owner: caller(),
            title,
            description,
            highest_bid: None,
            active: true,
        };
        state.add_item(item);
        item_id
    })
}

pub fn place_bid(item_id: u64, amount: u64) {
    with_state_mut(|state| {
        let item = state.get_item(item_id).expect("Item not found");

        if !item.active {
            ic_cdk::trap("Item is not active.");
        }

        if let Some(highest_bid) = &item.highest_bid {
            if amount <= highest_bid.amount {
                ic_cdk::trap("Bid must be higher than the current highest bid.");
            }
        }

        let bid = Bid { bidder: caller(), amount };
        state.bids.entry(item_id).or_default().push(bid.clone());

        let mut updated_item = item.clone();
        updated_item.highest_bid = Some(bid);
        state.update_item(updated_item);
    })
}

pub fn modify_item(item_id: u64, title: String, description: String) {
    with_state_mut(|state| {
        let item = state.get_item(item_id).expect("Item not found");

        if item.owner != caller() {
            ic_cdk::trap("Only the owner can update the item.");
        }

        let mut updated_item = item.clone();
        updated_item.title = title;
        updated_item.description = description;
        state.update_item(updated_item);
    })
}

pub fn stop_item_listing(item_id: u64) {
    with_state_mut(|state| {
        let item = state.get_item(item_id).expect("Item not found");

        if item.owner != caller() {
            ic_cdk::trap("Only the owner can stop the listing.");
        }

        let mut updated_item = item.clone();
        updated_item.active = false;
        state.update_item(updated_item);
    })
}

pub fn fetch_item(item_id: u64) -> Option<Item> {
    with_state(|state| state.get_item(item_id))
}

pub fn fetch_items() -> Vec<Item> {
    with_state(|state| state.items.clone())
}

pub fn fetch_item_count() -> usize {
    with_state(|state| state.items.len())
}

pub fn fetch_highest_bid(item_id: u64) -> Option<Bid> {
    with_state(|state| state.get_item(item_id).and_then(|item| item.highest_bid.clone()))
}

pub fn fetch_item_stats() -> (Option<Item>, Option<Item>) {
    with_state(|state| {
        let most_bids = state.items.iter()
            .max_by_key(|item| state.bids.get(&item.id).map(|b| b.len()).unwrap_or(0))
            .cloned();

        let highest_sold = state.items.iter()
            .max_by_key(|item| item.highest_bid.as_ref().map(|b| b.amount).unwrap_or(0))
            .cloned();

        (most_bids, highest_sold)
    })
}
