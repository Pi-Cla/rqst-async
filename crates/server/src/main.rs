use http::StatusCode;
use miniserve::{Content, Request, Response};
use serde::{Deserialize, Serialize};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

#[derive(Debug, Serialize, Deserialize)]
struct Cont {
    messages: Vec<String>,
}

fn chat(req: Request) -> Response {
    match req {
        Request::Get => Err(StatusCode::METHOD_NOT_ALLOWED),
        Request::Post(str) => {
            let mut cont: Cont =
                serde_json::from_str(str.as_str()).map_err(|_| StatusCode::BAD_REQUEST)?;
            cont.messages
                .push("How would you rate your pain?".to_string());
            Ok(Content::Json(
                serde_json::to_string(&cont).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            ))
        }
    }
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
