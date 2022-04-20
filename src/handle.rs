use std::collections::HashMap;

use crate::AppState;
use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use arrow::{json, record_batch::RecordBatch};

pub async fn get_all_pokemons(app_state: Data<AppState>) -> HttpResponse {
    let mut ctx = app_state.execution_context.clone();
    let df = ctx.sql("SELECT * FROM POKEMON;").await.unwrap();
    let results: Vec<RecordBatch> = df.collect().await.unwrap();
    let data = json::writer::record_batches_to_json_rows(&results).unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&data).unwrap())
}

pub async fn get_pokemon(
    app_state: Data<AppState>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let index = if let Some(index) = query.get("index") {
        let index = index.trim().to_string();
        if !index.is_empty() {
            index
        } else {
            String::from("0")
        }
    } else {
        String::from("0")
    };

    let name = if let Some(name) = query.get("name") {
        name.trim().to_string()
    } else {
        String::from("")
    };

    let query = format!(
        "SELECT * FROM POKEMON WHERE LOWER(\"Name\") = '{}' OR # = {};",
        name.to_lowercase(),
        index
    );
    let mut ctx = app_state.execution_context.clone();
    let df = ctx.sql(&query).await.unwrap();
    let results: Vec<RecordBatch> = df.collect().await.unwrap();
    let data = json::writer::record_batches_to_json_rows(&results)
        .unwrap()
        .first()
        .cloned();
    HttpResponse::Ok().body(serde_json::to_string(&data).unwrap())
}
