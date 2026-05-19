use std::{
    collections::{HashMap, VecDeque},
    default,
    fmt::format,
    sync::{Mutex, atomic::AtomicU64},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const RESERVATION_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Clone)]
pub struct Reservation {
    pub reservation_id: String,
    pub user_id: String,
    pub expires_at: DateTime<Utc>,
}

impl Reservation {
    pub fn new(user_id: String) -> Reservation {
        let id = RESERVATION_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            reservation_id: format!("res_{id}"),
            user_id: user_id,
            expires_at: Utc::now() + chrono::Duration::seconds(300),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        if self.expires_at < now {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Default)]
pub struct SalesState {
    pub inventory: u64,
    pub reservations: HashMap<String, Reservation>, //<user_id, Reservations>
    pub waitlist: VecDeque<String>,
}

impl SalesState {
    pub fn initialise_inventory(count: u64) -> SalesState {
        Self {
            inventory: count,
            reservations: HashMap::new(),
            waitlist: VecDeque::new(),
        }
    }

    // will have to check in the reservations hashmap and if any reservation has expired will
    // increment the inventory count and remove the user from the reservation hashmap
}

pub struct SharedState(pub Mutex<SalesState>);

impl SharedState {
    pub fn new(state: SalesState) -> Self {
        SharedState(Mutex::new(state))
    }
}

#[derive(Deserialize, Serialize)]
pub struct InitRequest {
    pub inventory: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ReserveRequest {
    pub user_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReserveSuccess {
    pub reservation_id: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct WaitlistResponse {
    pub message: &'static str,
    pub waitlist_position: usize,
}

#[derive(Deserialize, Serialize)]
pub struct StatusResponse {
    pub available_inventory: u64,
    pub waitlist_size: usize,
    pub active_reservations: usize,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
