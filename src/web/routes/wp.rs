// export the home route handler
use std::fs;
use actix_web::{web, Scope, get, HttpResponse, Responder};

#[tracing::instrument(level = "info")]
pub async fn main(path:String) -> HttpResponse {
 
    if path == "/vulns/wp-admin" || path == "/vulns/wp-login.php" {
        let index = fs::read_to_string("vulns/wp/login.html").unwrap();
        return HttpResponse::Ok().content_type("text/html").body(index);
    } else {
        return HttpResponse::Ok().content_type("text/plain").body("404 Not Found");
    }
}