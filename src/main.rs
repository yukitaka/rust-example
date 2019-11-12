extern crate actix;
extern crate actix_protobuf;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate prost;
extern crate prost_derive;

use actix_protobuf::*;
use actix_web::*;

pub mod my_obj {
    include!(concat!(env!("OUT_DIR"), "/rust_example.rs"));
}

fn index(msg: ProtoBuf<my_obj::MyObj>) -> Result<HttpResponse> {
    println!("model: {:?}", msg);
    HttpResponse::Ok().protobuf(msg.0) // <- send response
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("prost-example");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::post().to(index)))
    }).bind("0.0.0.0:8080")
    .unwrap()
    .shutdown_timeout(1)
    .start();

    println!("Started http server: 0.0.0.0:8080");
    let _ = sys.run();
}
