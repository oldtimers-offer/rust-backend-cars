extern crate diesel;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

mod models;
mod schema;

use models::{ClassicCar, NewClassicCar};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize)]
struct CreateCarRequest {
    make: String,
    model: String,
    year: i32,
    mileage: i32,
}

// GET all cars
async fn get_cars(pool: web::Data<DbPool>) -> HttpResponse {
    use schema::classic_cars::dsl::*;
    let mut conn = pool.get().unwrap();

    let cars = classic_cars
        .load::<ClassicCar>(&mut conn)
        .expect("Error loading cars");

    HttpResponse::Ok().json(cars)
}

// POST a new car
async fn add_car(pool: web::Data<DbPool>, car: web::Json<CreateCarRequest>) -> HttpResponse {
    use schema::classic_cars;

    let mut conn = pool.get().unwrap();
    let new_car = NewClassicCar {
        make: car.make.clone(),
        model: car.model.clone(),
        year: car.year,
        mileage: car.mileage,
    };

    diesel::insert_into(classic_cars::table)
        .values(&new_car)
        .execute(&mut conn)
        .expect("Error inserting car");

    HttpResponse::Ok().json("Car added successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/cars", web::get().to(get_cars))
            .route("/cars", web::post().to(add_car))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
