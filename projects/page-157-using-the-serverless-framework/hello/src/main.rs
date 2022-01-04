type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(lambda::handler_fn(hello)).await?;
    Ok(())
}

async fn hello(event: serde_json::Value, _: lambda::Context) -> Result<serde_json::Value, Error> {
    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hello_handles() {
        let event = serde_json::json!({
            "answer": 42
        });
        assert_eq!(
            hello(event.clone(), lambda::Context::default())
                .await
                .expect("expected Ok(_) value"),
            event
        )
    }
}
