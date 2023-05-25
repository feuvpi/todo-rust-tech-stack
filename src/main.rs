#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};

use std::{io::ErrorKind, sync::Arc};
use surrealdb::{sql::Object, Datastore, Session};

use crate::db::{AffectedRows, DB};

use cors::*;

mod cors;
mod db;
mod error;
mod prelude;
mod utils;

#[post("/task/<title>")]
async fn add_task(title: String, db: &State<DB>) -> Result<Json<Object>, std::io::Error> {
    let task = db
        .add_task(title)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create task."))?;

    Ok(Json(task))
}

#[get("/task/<id>")]
async fn get_task(id: String, db: &State<DB>) -> Result<Json<Object>, std::io::Error> {
    let task = db
        .get_task(id)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to fetch task."))?;

    Ok(Json(task))
}
