use actix_web::{web, App, HttpServer};

use actix_web::middleware::Logger;
use env_logger::Env;

mod controller;
mod service;

pub struct ServiceContainer {
    user: service::UserService,
}

impl ServiceContainer {
    pub fn new(user: service::UserService) -> Self {
        ServiceContainer { user }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let client_options =
        mongodb::options::ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = mongodb::sync::Client::with_options(client_options).unwrap();
    let db = client.database("nft");

    let user_collection = db.collection("nft_meta");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let service_container =
            ServiceContainer::new(service::UserService::new(user_collection.clone()));

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(AppState { service_container })
            .route("/hello", web::get().to(controller::index))
            .route("/get/{id}", web::get().to(controller::get))
    })
    .bind("0.0.0.0:8090")?
    .run()
    .await
}
