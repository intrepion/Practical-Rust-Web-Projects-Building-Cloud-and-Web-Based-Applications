use lambda_http::lambda;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(lambda_http::handler(world)).await?;
    Ok(())
}

async fn world(
    _: lambda_http::Request,
    _: lambda_http::Context,
) -> Result<impl lambda_http::IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(serde_json::json!({
    "message": "Go Serverless v1.0! Your function executed successfully!"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn world_handles() {
        let request = lambda::Request::default();
        let expected = json!({
        "message": "Go Serverless v1.0! Your function executed successfully!"
        })
        .into_response();
        let response = world(request, lambda::Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
