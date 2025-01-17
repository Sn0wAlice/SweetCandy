// export the home route handler
use std::fs;
use actix_web::{web, Scope, get, HttpResponse, Responder};

#[tracing::instrument(level = "info")]
pub async fn home() -> HttpResponse {
  let index = fs::read_to_string("html/home/index.html").unwrap();

  return HttpResponse::Ok().content_type("text/html").body(index);
}