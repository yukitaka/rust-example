use actix_protobuf::*;
use actix_web::*;

pub mod my_obj {
    include!(concat!(env!("OUT_DIR"), "/rust_example.rs"));
}

async fn index(msg: ProtoBuf<my_obj::MyObj>) -> Result<HttpResponse> {
    println!("model: {:?}", msg);
    HttpResponse::Ok().protobuf(msg.0) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::post().to(index)))
    }).bind("0.0.0.0:8080")
        .unwrap()
        .shutdown_timeout(1)
        .run()
        .await
}
