use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde::Deserialize;
use serde_json::{json, Value};

/// Lambda functions have their requests mapped to "Events", which include the HTTP body as a JSON string member
#[derive(Deserialize)]
struct EventPayload {
    body: String
}

/// The actual request model — contains the first name to use
#[derive(Deserialize)]
struct Request {
    first_name: String
}

/// Initialises the tracing subscriber for CloudWatch logging.
fn init_lambda_tracing() {
    tracing_subscriber::fmt()      
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_lambda_tracing();
    let service = service_fn(greeting_as_a_service);
    lambda_runtime::run(service).await?;
    Ok(())
}

async fn greeting_as_a_service(event: LambdaEvent<EventPayload>) -> Result<Value, Error> {
    let (payload, _context) = event.into_parts();

    // An Err() from a lambda function means an internal server error occurred. This does not provide the user with nice output (though it does log to CloudWatch).
    // For errors caused by invalid user input, we return an Ok() with an error message instead.
    Ok(match serde_json::from_str::<Request>(&payload.body) {
        Ok(request) => json!({ "message": format!("Hello, {}!", request.first_name) }),
        Err(cause) => json!({ "error": format!("{}", cause) })
    })
}