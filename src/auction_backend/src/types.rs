use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone)]
pub struct Item {
    pub id: u64,
    pub owner: Principal,
    pub title: String,
    pub description: String,
    pub highest_bid: Option<Bid>,
    pub active: bool,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Bid {
    pub bidder: Principal,
    pub amount: u64,
}
