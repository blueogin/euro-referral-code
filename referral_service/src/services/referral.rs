use actix_web::{web, Responder, HttpResponse};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct ReferralRequest {
    pub user_id: String,
    pub referral_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    message: String,
}

// Function to call the /validate_user/{user_id} API endpoint
async fn validate_user(pool: web::Data<PgPool>, user_id: &str) -> Result<bool, reqwest::Error> {
    // Making a GET request to the validate_user API endpoint
    let client = Client::new();
    let url = format!("http://localhost:8080/validate_user/{}", user_id);
    let response = client.get(&url)
        .timeout(Duration::from_secs(10)) // Set a timeout for the request
        .send()
        .await?;

    // Check if user is eligible based on the response
    if response.status().is_success() {
        let body: ApiResponse = response.json().await?;
        if body.message == "User is eligible" {
            return Ok(true);
        }
    }
    
    Ok(false)
}

// store_referral function with validation logic
pub async fn store_referral(pool: web::Data<PgPool>, data: web::Json<ReferralRequest>) -> impl Responder {
    println!("This is a log message!");
    
    // Call the validation function
    let is_eligible = validate_user(pool.clone(), &data.user_id).await.unwrap_or(false);

    if !is_eligible {
        return HttpResponse::BadRequest().json(ApiResponse { message: "User has reached referral limit".to_string() });
    }

    // Proceed with storing the referral if the user is eligible
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
