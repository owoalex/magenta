use actix_web::{get, web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize)]
struct RootInfo {
    instance_fqdn: String,
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    let obj = RootInfo {
        instance_fqdn: "takeshi.indentationerror.com".to_owned(),
    };
    let output_string = match serde_json::to_string(&obj) {
        Ok(json_string) => json_string,
        Err(error) => panic!("Unexpected error processing index: {:?}", error),
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .header("X-Hdr", "sample")
        .body(output_string)
}

#[derive(Serialize)]
struct ObjectInfo {
    requested_object: String,
}

#[get("/{object_id}")]
async fn get_object(object_id: web::Path<String>) -> impl Responder {
    let obj = ObjectInfo {
        requested_object: object_id.to_string(),
    };
    web::Json(obj)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("certs/privkey.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("certs/cert.pem").unwrap();

    println!("Server going up!");
    
    HttpServer::new(|| App::new().service(index).service(get_object))
        .bind_openssl("127.0.0.1:8443", builder)?
        .run()
        .await
}
