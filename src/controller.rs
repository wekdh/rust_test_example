use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct User {
    name: String,
    age: String
}

pub async fn index(app_data: web::Data<crate::AppState>, user: web::Query<User>) -> impl Responder {
    let result = web::block(move || app_data.service_container.user.create(&user.name, &user.age)).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get(app_data: web::Data<crate::AppState>, id: web::Path<String>) -> impl Responder {
    let result = web::block(move || app_data.service_container.user.get(&id)).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("get Error, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_by_name(app_data: web::Data<crate::AppState>, name: web::Path<String>) -> impl Responder {
    let result = web::block(move || app_data.service_container.user.get_by_name(&name)).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("get_by_name Error, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
