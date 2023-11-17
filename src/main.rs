use axum::{Json, Router, routing::get};
use axum::extract::Query;
use axum_valid::Valid;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

mod similarr;

fn valid_numbers(s: &String) -> Result<(), ValidationError> {
    let regex = Regex::new(r"\d{3,}+").unwrap();
    if regex.is_match(s) {
        Err(ValidationError::new("Strings support expansion up to 20 characters"))
    } else {
        Ok(())
    }
}

#[derive(Deserialize, Validate, Debug)]
struct ComparisonRequest {
    #[validate(length(max = 50))]
    #[validate(custom(function = "valid_numbers"))]
    a: String,
    #[validate(custom(function = "valid_numbers"))]
    #[validate(length(max = 50))]
    b: String,
}

#[derive(Debug, Serialize)]
struct ComparisonResponse {
    a: String,
    b: String,
    expanded_a: String,
    expanded_b: String,
    result: bool,
}

async fn compare(request: Valid<Query<ComparisonRequest>>) -> Json<ComparisonResponse> {
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
