use axum::{Json, Router, routing::get};
use axum::extract::Query;
use serde::{Deserialize, Serialize};

mod similarr;

#[derive(Deserialize)]
struct ComparisonRequest {
    a: String,
    b: String
}

#[derive(Debug, Serialize)]
struct ComparisonResponse {
    input: Vec<String>,
    expanded: Vec<String>,
    result: bool,
}

async fn compare(request: Query<ComparisonRequest>) -> Json<ComparisonResponse> {
    let result = similarr::compare(&request.a, &request.b);
    Json(ComparisonResponse {
        input: vec![request.a.to_string(), request.b.to_string()],
        expanded: result.expanded,
        result: result.result,
    })
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/compare", get(compare));
    Ok(router.into())
}
