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
    a: String,
    b: String,
    expanded_a: String,
    expanded_b: String,
    result: bool,
}

async fn compare(request: Query<ComparisonRequest>) -> Json<ComparisonResponse> {
    let result = similarr::compare(&request.a, &request.b);
    Json(ComparisonResponse {
        a: request.a.to_string(),
        b: request.b.to_string(),
        expanded_a: result.expanded_a,
        expanded_b: result.expanded_b,
        result: result.result,
    })
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/compare", get(compare));
    Ok(router.into())
}
