use lambda_http::http::{Response, StatusCode};
use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
use serde_json::json;
use std::collections::HashMap;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(cats)).await?;
    Ok(())
}

async fn cats(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let client = DynamoDbClient::new(Region::UsWest2);

    let scan_input = ScanInput {
        table_name: "oliver_catdex".to_string(),
        limit: Some(100),
        ..Default::default()
    };

    let response = match client.scan(scan_input).await {
        Ok(output) => match output.items {
            Some(items) => json!(items
                .into_iter()
                .map(|item| item.into_iter().map(|(k, v)| (k, v.s.unwrap())).collect())
                .collect::<Vec<HashMap<String, String>>>())
            .into_response(),
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("No cat yet".into())
                .expect("Failed to render response"),
        },
        Err(error) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("{:?}", error).into())
            .expect("Failed to render response"),
    };

    Ok(response)
}
