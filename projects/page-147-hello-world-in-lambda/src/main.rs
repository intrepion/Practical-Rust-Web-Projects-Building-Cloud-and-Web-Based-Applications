use simple_logger::SimpleLogger;

#[derive(serde_derive::Deserialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(serde_derive::Serialize)]
struct CustomOutput {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .with_utc_timestamps()
        .init()?;
    let func = lambda_runtime::handler_fn(my_handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn my_handler(
    e: CustomEvent,
    c: lambda_runtime::Context,
) -> Result<CustomOutput, lambda_runtime::Error> {
    if e.first_name == "" {
        log::error!("Empty first name in request {}", c.request_id);
        simple_error::bail!("Empty first name");
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}
