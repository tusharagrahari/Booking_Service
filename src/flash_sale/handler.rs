use std::{os::macos::raw::stat, sync::Arc};

use actix_web::{HttpResponse, web};

use crate::flash_sale::state::{
    self, ErrorResponse, InitRequest, Reservation, ReserveRequest, ReserveSuccess, SharedState,
    StatusResponse, WaitlistResponse,
};

pub async fn init_inventory(
    data: web::Data<Arc<SharedState>>,
    body: web::Json<InitRequest>,
) -> HttpResponse {
    let mut state = match data.0.lock() {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to initialize".into(),
            });
        }
    };

    state.inventory = body.inventory;
    state.reservations.clear();
    state.waitlist.clear();

    HttpResponse::Ok().json(serde_json::json!({
            "message": "Initialized",
            "inventory": state.inventory
    }))
}

pub async fn reserve(
    data: web::Data<Arc<SharedState>>,
    body: web::Json<ReserveRequest>,
) -> HttpResponse {
    if body.user_id.trim().is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "user_id must be present".into(),
        });
    }

    let mut state = match data.0.lock() {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "State lock poised".into(),
            });
        }
    };

    // now here we will call the function to check for expired reservations
    // before making a new reservation inorder to get the correct number of items in the inventory!

    if state.inventory > 0 {
        state.inventory -= 1;
        let reservation = Reservation::new(body.user_id.clone());
        let response = ReserveSuccess {
            reservation_id: reservation.reservation_id.clone(),
            expires_at: reservation.expires_at,
        };
        state
            .reservations
            .insert(reservation.reservation_id.clone(), reservation);
        return HttpResponse::Created().json(response);
    } else {
        let already_waiting = state.waitlist.contains(&body.user_id);
        if !already_waiting {
            state.waitlist.push_back(body.user_id.clone());
        }

        let position = state
            .waitlist
            .iter()
            .position(|id| id == &body.user_id)
            .map(|p| p + 1)
            .unwrap_or(0);

        let resp = WaitlistResponse {
            message: "Added to queue",
            waitlist_position: position,
        };

        return HttpResponse::Accepted().json(resp);
    }
}

pub async fn status(data: web::Data<Arc<SharedState>>) -> HttpResponse {
    let mut state = match data.0.lock() {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "State lock poised".into(),
            });
        }
    };

    let resp = StatusResponse {
        available_inventory: state.inventory,
        waitlist_size: state.waitlist.len(),
        active_reservations: state.reservations.len(),
    };
    return HttpResponse::Ok().json(resp);
}
