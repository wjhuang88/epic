use actix_web::{HttpServer, App, web, HttpResponse, HttpMessage, http, FromRequest, Responder};
use lazy_static::*;
use futures::StreamExt;
use mimalloc::MiMalloc;
use git2::Repository;
use git2::transport::Transport;
use actix_web::dev::Factory;
use std::future::Future;

#[allow(non_snake_case)]
mod gen___pages;
mod logger_config;

lazy_static! {
    static ref STATIC_MAP: gen___pages::GenPages<'static> = gen___pages::GenPages::new();
}

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    logger_config::print_banner();
    logger_config::init_log();

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
            .route("{project}.git/HEAD", web::get().to(get_head))
            .route("{project}.git/info/refs", web::get().to(get_info_refs))
            .route("{project}.git/objects/info/alternates", web::get().to(get_text_file))
            .route("{project}.git/objects/info/http-alternates", web::get().to(get_text_file))
            .route("{project}.git/objects/info/packs", web::get().to(get_info_packs))
            .route("{project}.git/objects/{pre}/{id}", web::get().to(get_loose_object))
            .route("{project}.git/objects/pack/pack-{id}\\.pack", web::get().to(get_pack_file))
            .route("{project}.git/objects/pack/pack-{id}\\.idx", web::get().to(get_idx_file))

            .route("/git-upload-pack", web::post().to(service_rpc))
            .route("/git-receive-pack", web::post().to(service_rpc))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn get_head(req: web::HttpRequest) -> impl Responder {
    let maybe_project = req.match_info().get("project");
    let git_repo = Repository::open("demo-git-repo").unwrap();
    println!("{}", maybe_project.unwrap());
    "get_head"
}
async fn get_info_refs(req: web::HttpRequest) -> impl Responder { "get_info_refs" }
async fn get_text_file(req: web::HttpRequest) -> impl Responder { "get_text_file" }
async fn get_info_packs(req: web::HttpRequest) -> impl Responder { "get_info_packs" }
async fn get_loose_object(req: web::HttpRequest) -> impl Responder { "get_loose_object" }
async fn get_pack_file(req: web::HttpRequest) -> impl Responder { "get_pack_file" }
async fn get_idx_file(req: web::HttpRequest) -> impl Responder { "get_idx_file" }

async fn service_rpc(req: web::HttpRequest) -> impl Responder { "service_rpc" }
