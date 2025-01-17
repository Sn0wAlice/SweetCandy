use actix_web::{web,Scope,get,post,HttpResponse, HttpRequest, Responder, CustomizeResponder};
use serde_json::Value;
use futures::StreamExt;
use crate::helper::trace::{trace_logs,trace_warn};

use crate::api::mods::*;

const MAX_SIZE: usize = 2_097_152; // max payload size is 2MB

#[derive(Debug, Clone)]
pub struct RequestData {
    // Request basic information
    pub path: String,
    pub user_ip: String,
    pub method: String,
}


#[post("/{path:.*}")]
pub async fn handlerpost(path: web::Path<String>, payload: Option<web::Payload>, req: HttpRequest) -> CustomizeResponder<HttpResponse> {

    let request_data = log_request(path.clone().into(), req.clone(), "POST").await;
    let request_body = get_request_body(payload.unwrap()).await;

    println!("Request: {:?}", request_data);
    println!("Body: {:?}", request_body);

    match request_data.path.to_string().as_str() {
        "" => {
            return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"OK\"}")
                .customize();
        }

        "/hello" => {
            return hello::hello(request_data, request_body).await;
        }

        _ => {
            trace_warn(format!("404 Not Found: {}", path.to_string()));
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": \"path not found\"}")
                .customize();
        }
    }
}



#[get("/{path:.*}")]
pub async fn handler(path: web::Path<String>, req: HttpRequest) -> impl Responder {

    let request_data = log_request(path.clone().into(), req.clone(), "GET").await;
    trace_logs(format!("api: {}", path.to_string()));

    match request_data.path.to_string().as_str() {
        "" => {
            return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"OK\"}");
        }

        _ => {
            trace_warn(format!("404 Not Found: {}", path.to_string()));
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": \"path not found\"}");
        }
    }
}


pub async fn log_request(path: web::Path<String>, req: HttpRequest, method:&str) -> RequestData {
    // get user IP address
    let connection_info = req.connection_info().clone();  // Bind the result of connection_info
    let user_ip = connection_info.realip_remote_addr();    // Use connection_info to get peer_addr
    let user_ip: String = match user_ip {
        Some(ip) => ip.to_string(),
        None => match connection_info.peer_addr() {
              Some(ip) => ip.to_string(),
              None => "unknown".to_string()
          }
    };

    trace_logs(format!("Request: {} - {}", method, path.to_string()));

    RequestData {
        path: format!("/{}", path.to_string()),
        method: method.to_string(),
        user_ip: user_ip,
    }
}

pub async fn get_request_body(mut payload: web::Payload) -> Value {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = match chunk {
            Ok(chunk) => chunk,
            Err(_) => {
                return Value::Null;
            }
        };
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Value::Null;
        }
        body.extend_from_slice(&chunk);
    }

    // Get the expected data
    let str_data = std::str::from_utf8(&body).expect("Invalid UTF-8");
    let parsed_json: Value = serde_json::from_str(str_data).unwrap_or(Value::Null);

    return parsed_json;
}


pub fn init_api() -> Scope {
    web::scope("/api").service(handler).service(handlerpost)
}
