extern crate sweetcandy;
use std::fs;
use actix_cors::Cors;
use actix_web::{HttpServer,App};
use sweetcandy::{api,web,assets};

// init the tracing module
use sweetcandy::helper::trace::{init_trace,trace_logs};
use sweetcandy::helper::start::start;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", fs::read_to_string("utils/ascii.art").unwrap().as_str());
    init_trace();
    trace_logs("Sorry for the inconvenience, the log will be displayed in japanese.".to_string());
    trace_logs("サーバーが起動しています...".to_string());
    start().await;
    
    let port: u16 = 8080;
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        App::new().wrap(cors).service(api::init::init_api()).service(assets::init::init_assets()).service(web::init::init_web())
    }).bind(("0.0.0.0",port))?.run().await
}
