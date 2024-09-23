use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryParam {
    message: String, 
    id: u32,
}

pub async fn query_params(Query(query): Query<QueryParam>) -> Json<QueryParam> {
    Json(query)
}