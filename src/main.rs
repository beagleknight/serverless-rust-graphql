use lambda_http::{lambda, Body, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;
#[macro_use]
extern crate juniper;
use juniper::{EmptyMutation, Variables};

mod graphql;
use crate::graphql::{Query, Schema};

mod models;

fn main() {
    lambda!(handler)
}

fn handler(req: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let ctx = graphql::Context {};

    match req.body() {
        Body::Text(query) => {
            let (res, _errors) = juniper::execute(
                query,
                None,
                &Schema::new(Query, EmptyMutation::new()),
                &Variables::new(),
                &ctx,
            )
            .unwrap();

            // `serde_json::Values` impl `IntoResponse` by default
            // creating a application/json response
            Ok(json!({ "data": res.as_object_value().unwrap() }))
        }
        Body::Empty => Ok(json!({ "error": "Query not found" })),
        _ => Ok(json!({ "error": "The request body cannot be parsed" })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_handles() {
        let request = Request::new(Body::from("{ post(id: \"1\") { id title } }"));

        let expected = json!({
            "data": {
                "post": {
                    "id": "1",
                    "title": "First post"
                }
            }
        })
        .into_response();
        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
