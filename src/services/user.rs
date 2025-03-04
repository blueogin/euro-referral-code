use actix_web::{web, Responder, HttpResponse};
use sqlx::PgPool;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    message: String,
}

pub async fn validate_user_referrals(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
    let count = sqlx::query!("SELECT COUNT(*) as count FROM referrals WHERE user_id = $1", user_id.as_str())
        .fetch_one(pool.get_ref())
        .await;

    match count {
        Ok(record) if record.count.unwrap_or(0) < 10 => HttpResponse::Ok().json(ApiResponse { message: "User is eligible".to_string() }),
        Ok(_) => HttpResponse::BadRequest().json(ApiResponse { message: "User has reached referral limit".to_string() }),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse { message: "Error checking referral count".to_string() }),
    }
}
