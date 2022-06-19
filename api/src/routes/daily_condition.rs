use crate::auth::claims::Claims;
use actix_web::{web, HttpResponse};
use domain::interface::DailyConditionUsecase;
use domain::interface::StoreDailyConditionParams;
use domain::usecase::DailyConditionInteractor;
use infra::{DailyConditionDriverImpl, DailyConditionGateway};
use serde::Serialize;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub weight: i32,
    pub sleep_time: i32,
    pub mental_score: i32,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn store_daily_condition(
    _claims: Claims,
    pool: web::Data<PgPool>,
    form: web::Json<FormData>,
) -> HttpResponse {
    let connection_pool = pool.into_inner();
    let daily_condition_port = DailyConditionGateway {
        daily_condition_driver: DailyConditionDriverImpl {
            pool: connection_pool,
        },
    };
    let interactor = DailyConditionInteractor;
    let params = StoreDailyConditionParams {
        weight: form.weight,
        sleep_time: form.sleep_time,
        mental_score: form.mental_score,
    };
    return match interactor
        .store_daily_condition(&daily_condition_port, &params)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        _ => HttpResponse::BadRequest().json(ErrorResponse {
            message: "Failed to store daily condition".to_string(),
        }),
    };
}
