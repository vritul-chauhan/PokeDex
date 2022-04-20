mod handle;
mod routes;
mod settings;

use actix_web::{web, App, HttpServer};
use datafusion::prelude::{CsvReadOptions, ExecutionContext};
use routes::get_routes;
use settings::SETTING;

#[derive(Clone)]
pub struct AppState {
    execution_context: ExecutionContext,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = AppState {
        execution_context: register_data().await,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .configure(get_routes)
    })
    .bind((SETTING.SERVER.host.to_owned(), SETTING.SERVER.port))?
    .run()
    .await?;
    Ok(())
}

async fn register_data() -> ExecutionContext {
    let mut ctx = ExecutionContext::new();
    ctx.register_csv("POKEMON", "./data/Pokemon.csv", CsvReadOptions::new())
        .await
        .unwrap();
    ctx
}
