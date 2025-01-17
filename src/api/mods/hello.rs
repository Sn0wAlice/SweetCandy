use serde_json::Value;
use actix_web::{CustomizeResponder, HttpResponse, Responder};
use crate::helper::functions::{control_body, is_uuid_v4, is_valid_email, sha512_string, extract_string_from_obj_value};
use crate::api::init::RequestData;


pub async fn hello(request_data:RequestData, request_body: Value) ->  CustomizeResponder<HttpResponse> {

    if request_body.is_null() || !control_body(vec!["email"], &request_body) {
        return HttpResponse::Ok().content_type("application/json")
            .body("{\"error\": \"missing_field\", \"message\": \"'email' field is required\"}")
            .customize();
    }

    let email = extract_string_from_obj_value(request_body.get("email"));

    if !is_valid_email(&email) {
        return HttpResponse::Ok().content_type("application/json")
            .body("{\"error\": \"invalid_email\", \"message\": \"email is not valid\"}")
            .customize();
    }

    return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"success\"}")
        .customize();
}