use axum::{extract::State, Json};
use jiff::Timestamp;
use serde::Serialize;

use crate::web::AppState;

#[derive(Serialize)]
pub struct HealthCheck {
    pub uptime: String,
}

pub async fn health_check(State(state): State<AppState>) -> Json<HealthCheck> {
    let now = Timestamp::now();
    let uptime = now - *state.launch_time;
    let uptime = format!("{uptime:#}");

    Json(HealthCheck { uptime })
}
