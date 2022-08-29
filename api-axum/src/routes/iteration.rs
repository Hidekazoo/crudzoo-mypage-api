use crate::auth::claims::Claims;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use axum_macros::debug_handler;
use chrono::{DateTime, Utc};
use domain::{entity::iteration::Iteration, usecase::iteration::add_iteration};
use infra::{iteration::IterationService, IterationDriver};
use serde::{Deserialize};
use sqlx::PgPool;

#[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct PostIteration {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    hours: i32,
}

#[debug_handler]
pub async fn post_iteration(
    _claims: Claims,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<PostIteration>,
) -> impl IntoResponse {
    let start_date = payload.start_date;
    let end_date = payload.end_date;
    let iteration = Iteration::new(start_date, end_date, payload.hours);
    let iteration_driver = IterationDriver { pool };
    let iteration_service = IterationService { iteration_driver };
    match add_iteration(iteration_service, iteration).await {
        Ok(_) => return StatusCode::ACCEPTED,
        _ => return StatusCode::INTERNAL_SERVER_ERROR,
    }
}
