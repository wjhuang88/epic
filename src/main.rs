use actix_web::{HttpServer, App, web, HttpResponse};
use lazy_static::*;

#[allow(non_snake_case)]
mod gen___pages;

lazy_static! {
    static ref STATIC_MAP: gen___pages::GenPages<'static> = gen___pages::GenPages::new();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let mut app = App::new();
        for (&k, &v) in &STATIC_MAP.map {
            let mime_type = if k.ends_with(".css") {
                mime::TEXT_CSS
            } else if k.ends_with(".js") {
                mime::APPLICATION_JAVASCRIPT_UTF_8
            } else if k.ends_with(".svg") {
                mime::IMAGE_SVG
            } else if k.ends_with(".html") {
                mime::TEXT_HTML_UTF_8
            } else if k.ends_with(".json") {
                mime::APPLICATION_JSON
            } else if k.ends_with(".png") {
                mime::IMAGE_PNG
            } else {
                mime::TEXT_PLAIN
            }.to_string();
            let body = String::from(v);
            if k == "/index.html" {
                let mime_type = mime_type.clone();
                let body = body.clone();
                app = app.route("/", web::get().to(move || {
                    HttpResponse::Ok().content_type(mime_type.clone()).body(body.clone())
                }));
            }
            app = app.route(k, web::get().to(move || {
                HttpResponse::Ok().content_type(mime_type.clone()).body(body.clone())
            }));
        }
        app
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
