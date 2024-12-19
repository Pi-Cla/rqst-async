use std::sync::Arc;

use http::StatusCode;
use miniserve::{Content, Request, Response};
use serde::{Deserialize, Serialize};

async fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

#[derive(Debug, Serialize, Deserialize)]
struct Cont {
    messages: Vec<String>,
}

async fn chat(req: Request) -> Response {
    match req {
        Request::Get => Err(StatusCode::METHOD_NOT_ALLOWED),
        Request::Post(str) => {
            // Deserialize the JSON into a Cont
            let mut cont: Cont =
                serde_json::from_str(str.as_str()).map_err(|_| StatusCode::BAD_REQUEST)?;

            // Make an Arc out of the existing cont.message data and another pointer (strong reference) to that Arc
            let messages = Arc::new(cont.messages);
            let messages_ref = Arc::clone(&messages);

            // Spawn a new thread for chatbot::query_chat(), note that messages_ref gets moved
            let possible = tokio::spawn(async move { chatbot::query_chat(&messages_ref).await });
            let num = chatbot::gen_random_number();
            let (possible, num) = tokio::join!(possible, num);
            let possible = possible.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Set cont.messages to the value of the messages after this is all done
            // This needs to be done because we had already moved the original cont.messages
            // into_inner only returns the inner value if there is exactly one "strong reference" else it would be None
            cont.messages =
                Arc::into_inner(messages).expect("into_inner should always succeed here.");
            cont.messages.push(possible[num % 2].clone());
            Ok(Content::Json(
                serde_json::to_string(&cont).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            ))
        }
    }
}

#[tokio::main]
async fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
        .await
}
