use actix_web::{web, Responder, HttpResponse};
use sqlx::PgPool;
use serde::Serialize;
use sqlx::Row;

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    message: String,
}

pub async fn validate_user_referrals(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
    println!("This is a validate_user_referrals log message! user_id = {}", user_id);
    let count_result = sqlx::query("SELECT COUNT(*) as count FROM referrals WHERE user_id = $1")
        .bind(user_id.as_str().clone())
        .fetch_one(pool.get_ref())
        .await;


    match count_result {
        Ok(row) => {
            let count: i64 = row.get(0); // Get the first column (COUNT)
            if count < 10 {
                HttpResponse::Ok().json(ApiResponse { message: "User is eligible".to_string() })
            } else {
                HttpResponse::BadRequest().json(ApiResponse { message: "User has reached referral limit".to_string() })
            }
        },
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse { message: "Error checking referral count".to_string() }),
    }
}
