use actix_web::{web, Responder, HttpResponse};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ReferralRequest {
    pub user_id: String,
    pub referral_code: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    message: String,
}

pub async fn store_referral(pool: web::Data<PgPool>, data: web::Json<ReferralRequest>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO referrals (user_id, referral_code) VALUES ($1, $2) ON CONFLICT (referral_code) DO NOTHING",
        data.user_id,
        data.referral_code
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse { message: "Referral stored successfully".to_string() }),
        Err(_) => HttpResponse::BadRequest().json(ApiResponse { message: "Failed to store referral".to_string() }),
    }
}
