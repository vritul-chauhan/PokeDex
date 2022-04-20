use crate::AppState;
use actix_web::{web::Data, HttpResponse};
use arrow::{json, record_batch::RecordBatch};

pub async fn get_all_pokemons(app_state: Data<AppState>) -> HttpResponse {
    let start = std::time::Instant::now();
    let mut ctx = app_state.execution_context.clone();
    let df = ctx.sql("select * from POKEMON").await.unwrap();
    let results: Vec<RecordBatch> = df.collect().await.unwrap();
    let data = json::writer::record_batches_to_json_rows(&results).unwrap();
    println!("time elapsed : {}", start.elapsed().as_secs_f32());
    HttpResponse::Ok().body(serde_json::to_string(&data).unwrap())
}

// pub async fn get_pokemon(app_state: Data<Arc<RwLock<AppState>>>) {

// }
