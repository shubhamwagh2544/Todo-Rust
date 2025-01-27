use actix_web::{web, App, HttpServer, HttpRequest, Responder, HttpResponse};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::Mutex;
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize, Clone)]
struct TodoItem {
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

fn main() {

}
