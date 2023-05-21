use actix_web::{web, App, HttpServer};
use middleware::UserAuthentication;
use sample_gql::GraphQLAppExt;
use sample_sql::MySqlPool;
use std::env;

async fn setup_mysql() -> MySqlPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to MySQL")
}

#[actix_web::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let db_pool = setup_mysql().await;
    HttpServer::new(move || {
        App::new()
            .wrap(UserAuthentication::new(db_pool.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .configure_graphql_api(db_pool.clone())
    })
    .bind("0.0.0.0:8088")
    .expect("Failed to bind to 0.0.0.0:8088")
    .run()
    .await
    .expect("Failed to run server");
}
