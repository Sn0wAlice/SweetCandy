use actix_web::{get,web,HttpResponse,HttpRequest,Responder};
use actix_web::http::StatusCode;
use std::fs;
use crate::helper::{find_insert::find_insert,replace_in_body::replace_in_body};
use crate::api::init::{RequestData, log_request};

// import the routes pages
use crate::web::routes::*;

#[get("/{path:.*}")]
#[tracing::instrument(level = "info", name = "Dispatch request", skip(path, req))]
pub async fn dispatch(path: web::Path<String>, req: HttpRequest) -> impl Responder {

  let request_data = log_request(path, req.clone(), "GET").await;
  println!("要求データ: {:?}", request_data);
  let content_body = nonlogged(request_data).await;

  return content_body;
}


pub async fn nonlogged(request_data:RequestData) -> HttpResponse {

  match request_data.path.as_str() {
    "/" => { return home::home().await; },


    // Vuln part
    path if path.starts_with("/vulns/.git/") => {
      return git::get_assets(request_data.path).await;
    },

  
    // default route: 404
    _ => {      
      return HttpResponse::Ok().content_type("text/plain").status(StatusCode::NOT_FOUND).body("404 Not Found");
    }
  }
}