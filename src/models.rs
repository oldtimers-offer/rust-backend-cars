use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::classic_cars;

#[derive(Queryable, Serialize, Deserialize)]
pub struct ClassicCar {
    pub id: i32,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub mileage: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=classic_cars)]
pub struct NewClassicCar {
    pub make: String,
    pub model: String,
    pub year: i32,
    pub mileage: i32,
}
