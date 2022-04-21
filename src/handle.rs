use std::collections::HashMap;

use crate::filter::{FilterValue, PokeFilter};
use crate::AppState;
use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use arrow::{json, record_batch::RecordBatch};
use itertools::Itertools;

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
    let index = if let Some(query_index) = query.get("index") {
        let index = query_index.trim().to_string();
        if !index.is_empty() {
            Some(index)
        } else {
            None
        }
    } else {
        None
    };

    let name = if let Some(query_name) = query.get("name") {
        let name = query_name.trim().to_string();
        if !name.is_empty() {
            Some(name)
        } else {
            None
        }
    } else {
        None
    };

    let query = if index.is_some() {
        format!("SELECT * FROM POKEMON WHERE # = {}", index.unwrap())
    } else if name.is_some() {
        format!(
            "SELECT * FROM POKEMON WHERE LOWER(\"Name\") = '{}'",
            name.unwrap().to_lowercase()
        )
    } else {
        return HttpResponse::BadRequest().body("'Name' and 'Index' Missing!");
    };

    let mut ctx = app_state.execution_context.clone();
    let df = ctx.sql(&query).await.unwrap();
    let results: Vec<RecordBatch> = df.collect().await.unwrap();
    let data = json::writer::record_batches_to_json_rows(&results)
        .unwrap()
        .first()
        .cloned();
    HttpResponse::Ok().body(serde_json::to_string(&data).unwrap())
}

pub async fn search_pokemon(
    app_state: Data<AppState>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let mut filters = Vec::new();

    if let Some(name) = query.get("name") {
        let data = name.trim().to_string();
        if !data.is_empty() {
            filters.push(PokeFilter {
                stat: "Name".to_string(),
                value: FilterValue::Like(data.to_string()),
            })
        }
    }

    let where_condition = filters
        .into_iter()
        .map(|filter| match filter.value {
            FilterValue::Equal(val) => format!("\"{}\" = '{}'", filter.stat, val),
            FilterValue::NotEqual(val) => format!("\"{}\" <> '{}'", filter.stat, val),
            FilterValue::Like(val) => format!("LOWER(\"{}\") LIKE '%{}%'", filter.stat, val),
            FilterValue::NotLike(val) => format!("LOWER(\"{}\") NOT LIKE '%{}%'", filter.stat, val),
        })
        .join(" AND ");

    let query = format!("SELECT * FROM POKEMON {};", where_condition);
    let mut ctx = app_state.execution_context.clone();
    let df = ctx.sql(&query).await.unwrap();
    let results: Vec<RecordBatch> = df.collect().await.unwrap();
    let data = json::writer::record_batches_to_json_rows(&results).unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&data).unwrap())
}
