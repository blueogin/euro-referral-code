mod services;
mod db;

use actix_web::{web, App, HttpServer};
use services::referral::store_referral;
use services::user::validate_user_referrals;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/store_referral", web::post().to(store_referral))
            .route("/validate_user/{user_id}", web::get().to(validate_user_referrals))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
