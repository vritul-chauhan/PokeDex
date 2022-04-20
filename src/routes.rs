use actix_web::{web, HttpResponse};

use crate::handle::get_all_pokemons;

async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("Server is running")
}

pub fn get_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(ping)))
        .service(
            web::scope("/getPokemon").route("/getAllPokemons", web::get().to(get_all_pokemons)),
        );
}
