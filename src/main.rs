use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tokio::task::spawn_blocking;

use rust_imagelib::{errors::Errors, image_to_bytes, ImageOperator};
use image::ImageFormat;
fn img_blocking(operator: ImageOperator, format: &str) -> Result<Vec<u8>, Errors> {
    let image = operator.apply_all_operations()?.get_image().unwrap();
    image_to_bytes(image, ImageFormat::from_extension(format).ok_or(Errors::InvalidImageType)?.into())
}

#[post("/img/{format}")]
async fn img(format: web::Path<String>, operator: web::Json<ImageOperator>) -> impl Responder {
    HttpResponse::Ok().body(
        spawn_blocking(move || img_blocking(operator.0, &format))
            .await
            .unwrap()
            .unwrap(),
    )
}


#[get("/version")]
async fn versions() -> impl Responder {
    HttpResponse::Ok().body(rust_imagelib::build_info::version_str())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = ("0.0.0.0", 8080);
    println!("Starting server at http://{}:{}", addr.0, addr.1);
    HttpServer::new(|| App::new().service(img))
        .bind(addr)?
        .run()
        .await
}
