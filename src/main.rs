use actix_files::Files;
use actix_web::{get, web, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Safi - website underconstruction."
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        // cfg.service(hello_world);
        cfg.service(Files::new("/assets/", "assets"));
        cfg.service(web::redirect("/", "/assets/index.html"));
    };

    Ok(config.into())
}
