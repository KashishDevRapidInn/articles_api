use actix_web::{web, App, HttpServer};
mod db_utils;
mod db_models;
mod services;
pub mod schema;

use db_utils::establish_connection;
use services::{test_connection, fetch_users, create_user,create_article,fetch_articles};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(test_connection)
            .service(fetch_users)
            .service(create_user)
            .service(create_article)
            .service(fetch_articles)
            // .service(fetch_user_articles)
            // .service(create_user_article)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
