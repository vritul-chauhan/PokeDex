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
    HttpResponse::Ok().json(serde_json::to_string(&data).unwrap())
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
    HttpResponse::Ok().json(serde_json::to_string(&data).unwrap())
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

    if let Some(index) = query.get("index") {
        let data = index.trim().to_string();
        if !data.is_empty() {
            filters.push(PokeFilter {
                stat: "#".to_string(),
                value: FilterValue::NumEqual(data.to_string()),
            })
        }
    }

    if let Some(r#type) = query.get("type") {
        let data = r#type.trim().to_string().to_lowercase();
        if !data.is_empty() {
            if !data.contains(',') {
                filters.push(PokeFilter {
                    stat: "".to_string(),
                    value: FilterValue::OneType(data.to_string()),
                })
            } else {
                filters.push(PokeFilter {
                    stat: "".to_string(),
                    value: FilterValue::TwoType(data.to_string()),
                })
            }
        }
    }

    if let Some(generation) = query.get("generation") {
        let data = generation.trim().to_string();
        if !data.is_empty() {
            filters.push(PokeFilter {
                stat: "Generation".to_string(),
                value: FilterValue::NumEqual(data.to_string()),
            })
        }
    }

    if let Some(legendary) = query.get("legendary") {
        let data = legendary.trim().to_string();
        if !data.is_empty() {
            filters.push(PokeFilter {
                stat: "Legendary".to_string(),
                value: FilterValue::StrEqual(data.to_string()),
            })
        }
    }

    if let Some(hp) = query.get("hp") {
        let data = hp.trim().to_string().to_lowercase();
        if !data.is_empty() {
            if data.starts_with("gt") {
                filters.push(PokeFilter {
                    stat: "HP".to_string(),
                    value: FilterValue::GreaterThan(data.trim_start_matches("gt").to_string()),
                })
            }
            if data.starts_with("lt") {
                filters.push(PokeFilter {
                    stat: "HP".to_string(),
                    value: FilterValue::LesserThan(data.trim_start_matches("lt").to_string()),
                })
            }
        }
    }

    if let Some(attack) = query.get("attack") {
        let data = attack.trim().to_string().to_lowercase();
        if !data.is_empty() {
            if data.starts_with("gt") {
                filters.push(PokeFilter {
                    stat: "Attack".to_string(),
                    value: FilterValue::GreaterThan(data.trim_start_matches("gt").to_string()),
                })
            }
            if data.starts_with("lt") {
                filters.push(PokeFilter {
                    stat: "Attack".to_string(),
                    value: FilterValue::LesserThan(data.trim_start_matches("lt").to_string()),
                })
            }
        }
    }

    if let Some(defense) = query.get("defense") {
        let data = defense.trim().to_string().to_lowercase();
        if !data.is_empty() {
            if data.starts_with("gt") {
                filters.push(PokeFilter {
                    stat: "Defense".to_string(),
                    value: FilterValue::GreaterThan(data.trim_start_matches("gt").to_string()),
                })
            }
            if data.starts_with("lt") {
                filters.push(PokeFilter {
                    stat: "Defense".to_string(),
                    value: FilterValue::LesserThan(data.trim_start_matches("lt").to_string()),
                })
            }
        }
    }

    if let Some(speed) = query.get("speed") {
        let data = speed.trim().to_string().to_lowercase();
        if !data.is_empty() {
            if data.starts_with("gt") {
                filters.push(PokeFilter {
                    stat: "Speed".to_string(),
                    value: FilterValue::GreaterThan(data.trim_start_matches("gt").to_string()),
                })
            }
            if data.starts_with("lt") {
                filters.push(PokeFilter {
                    stat: "Speed".to_string(),
                    value: FilterValue::LesserThan(data.trim_start_matches("lt").to_string()),
                })
            }
        }
    }

    if let Some(special_attack) = query.get("spAttack") {
        let data = special_attack.trim().to_string().to_lowercase();
        if !data.is_empty() {
            if data.starts_with("gt") {
                filters.push(PokeFilter {
                    stat: "Sp. Atk".to_string(),
                    value: FilterValue::GreaterThan(data.trim_start_matches("gt").to_string()),
                })
            }
            if data.starts_with("lt") {
                filters.push(PokeFilter {
                    stat: "Sp. Atk".to_string(),
                    value: FilterValue::LesserThan(data.trim_start_matches("lt").to_string()),
                })
            }
        }
    }

    if let Some(special_defense) = query.get("spDefense") {
        let data = special_defense.trim().to_string().to_lowercase();
        if !data.is_empty() {
            if data.starts_with("gt") {
                filters.push(PokeFilter {
                    stat: "Sp. Def".to_string(),
                    value: FilterValue::GreaterThan(data.trim_start_matches("gt").to_string()),
                })
            }
            if data.starts_with("lt") {
                filters.push(PokeFilter {
                    stat: "Sp. Def".to_string(),
                    value: FilterValue::LesserThan(data.trim_start_matches("lt").to_string()),
                })
            }
        }
    }

    let where_condition = filters
        .into_iter()
        .map(|filter| match filter.value {
            FilterValue::StrEqual(val) => format!("\"{}\" = '{}'", filter.stat, val),
            FilterValue::NumEqual(val) => format!("\"{}\" = {}", filter.stat, val),
            FilterValue::Like(val) => format!("LOWER(\"{}\") LIKE '%{}%'", filter.stat, val),
            FilterValue::GreaterThan(val) => format!("\"{}\" >= {}", filter.stat, val),
            FilterValue::LesserThan(val) => format!("\"{}\" <= {}", filter.stat, val),
            FilterValue::OneType(val) => format!("( LOWER(\"Type 1\") = '{0}' OR LOWER(\"Type 2\") = '{0}' )", val),
            FilterValue::TwoType(val) => {
                // let types = val.splitn(2, ',');
                // let t1 = types.next().unwrap();
                // let t2 = types.next().unwrap();
                let types = val.splitn(2, ',').collect_vec();
                format!("( LOWER(\"Type 1\") = '{0}' AND LOWER(\"Type 2\") = '{1}' ) OR ( LOWER(\"Type 1\") = '{1}' AND LOWER(\"Type 2\") = '{0}' )", types[0], types[1])
            }
        })
        .join(" AND ");

    let query = format!("SELECT * FROM POKEMON WHERE {};", where_condition);
    let mut ctx = app_state.execution_context.clone();
    let df = ctx.sql(&query).await.unwrap();
    let results: Vec<RecordBatch> = df.collect().await.unwrap();
    let data = json::writer::record_batches_to_json_rows(&results).unwrap();
    HttpResponse::Ok().json(data)
}
