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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .with_utc_timestamps()
        .init()?;
    lambda_runtime::lambda!(my_handler);

    Ok(())
}

fn my_handler(
    e: CustomEvent,
    c: lambda_runtime::Context,
) -> Result<CustomOutput, lambda_runtime::error::HandlerError> {
    if e.first_name == "" {
        log::error!("Empty first name in request {}", c.aws_request_id);
        simple_error::bail!("Empty first name");
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}
