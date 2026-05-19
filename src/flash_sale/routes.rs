use actix_web::web::{self, service};
use serde_json::ser;

use crate::flash_sale::handler::{init_inventory, reserve, status};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/admin/init", web::post().to(init_inventory))
            .route("/reserve", web::post().to(reserve))
            .route("/status", web::get().to(status)),
    );
}
