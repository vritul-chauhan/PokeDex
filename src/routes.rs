use actix_web::{web, HttpResponse};

use crate::handle::{get_all_pokemons, get_pokemon, search_pokemon};

async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("Server is running")
}

pub fn get_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(ping)))
        .service(
            web::scope("/pokemon")
                .route("/getAllPokemons", web::get().to(get_all_pokemons))
                .route("/getPokemon", web::get().to(get_pokemon))
                .route("/searchPokemon", web::get().to(search_pokemon)),
        );
}
