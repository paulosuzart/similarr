use axum::{Json, Router, routing::get};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_valid::Valid;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

mod similarr;

fn valid_numbers(s: &String) -> Result<(), ValidationError> {
    let regex = Regex::new(r"([3-9]\d+|\d{3,})").unwrap();
    if regex.is_match(s) {
        Err(ValidationError::new("Strings support expansion up to 29 characters"))
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

fn init_router() -> Router {
    Router::new().route("/compare", get(compare))
        .fallback(handler_404)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(init_router().into())
}


#[cfg(test)]
mod test {
    use std::iter;
    use axum_test::{TestServer, TestServerConfig};
    use super::*;

    #[tokio::test]
    async fn black_box_truthy() {
        let app = init_router();
        let config = TestServerConfig::builder()
            .default_content_type("application/json")
            .build();
        let server = TestServer::new_with_config(app, config).unwrap();

        let response = server.get("/compare")
            .add_query_param("a", "casa")
            .add_query_param("b", "ca1a").await;

        response.assert_json(&ComparisonResponse {
            a: "casa".to_string(),
            b: "ca1a".to_string(),
            expanded_a: "casa".to_string(),
            expanded_b: "ca*a".to_string(),
            result: true,
        });
    }

    #[tokio::test]
    async fn black_box_false() {
        let app = init_router();
        let config = TestServerConfig::builder()
            .default_content_type("application/json")
            .build();
        let server = TestServer::new_with_config(app, config).unwrap();

        let response = server.get("/compare")
            .add_query_param("a", "mangos")
            .add_query_param("b", "m1n1o").await;

        let payload : ComparisonResponse = response.json();
        assert!(!payload.result);
    }

    #[tokio::test]
    async fn black_box_invalid_notation() {
        let app = init_router();
        let config = TestServerConfig::builder()
            .default_content_type("application/json")
            .build();
        let server = TestServer::new_with_config(app, config).unwrap();

        let response = server.get("/compare")
            .add_query_param("a", "mangos")
            .add_query_param("b", "mang44s").await;
        response.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn black_box_invalid_size() {
        let app = init_router();
        let config = TestServerConfig::builder()
            .default_content_type("application/json")
            .build();
        let server = TestServer::new_with_config(app, config).unwrap();

        let mut string_a : String = "mango".to_string();
        string_a.extend(iter::repeat("s").take(45));
        let response = server.get("/compare")
            .add_query_param("a", string_a)
            .add_query_param("b", "mang44s").await;
        response.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_not_found() {
        let app = init_router();
        let config = TestServerConfig::builder()
            .default_content_type("application/json")
            .build();
        let server = TestServer::new_with_config(app, config).unwrap();

        let response = server.get("/").await;
        response.assert_status(StatusCode::NOT_FOUND);
    }
}