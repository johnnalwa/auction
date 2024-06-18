use ic_cdk::storage;
use ic_stable_structures::{StableBTreeMap, Storable};
use std::cell::RefCell;
use crate::types::{Item, Bid};

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    pub items: Vec<Item>,
    pub bids: StableBTreeMap<u64, Vec<Bid>>, // item_id -> bids
}

impl State {
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn update_item(&mut self, item: Item) {
        if let Some(i) = self.items.iter_mut().find(|i| i.id == item.id) {
            *i = item;
        }
    }

    pub fn get_item(&self, item_id: u64) -> Option<Item> {
        self.items.iter().find(|i| i.id == item_id).cloned()
    }
}

pub fn initialize() {
    STATE.with(|s| *s.borrow_mut() = State::default());
}

pub fn pre_upgrade() {
    STATE.with(|s| storage::stable_save((s.borrow().clone(),)).unwrap());
}

pub fn post_upgrade() {
    STATE.with(|s| *s.borrow_mut() = storage::stable_restore().unwrap());
}

pub fn with_state<T>(f: impl FnOnce(&State) -> T) -> T {
    STATE.with(|s| f(&s.borrow()))
}

pub fn with_state_mut<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with(|s| f(&mut s.borrow_mut()))
}
