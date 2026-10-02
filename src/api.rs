use leptos::prelude::*;
use leptos::server_fn::codec::JsonEncoding;
use leptos::server_fn::{BoxedStream, Websocket};
use serde::{Deserialize, Serialize};

/// One item on the assistant stream. `Done` ends the turn so the page can
/// drop the typing indicator.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChatEvent {
    Token(String),
    Done,
}

/// Chat over one WebSocket. Each user message arrives on `input`; assistant
/// tokens leave on the returned stream. History stays on this connection.
#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
pub async fn chat(
    input: BoxedStream<String, ServerFnError>,
) -> Result<BoxedStream<ChatEvent, ServerFnError>, ServerFnError> {
    use futures::StreamExt;

    // The upgrade has already happened. Pull the shared model from Actix
    // app data before returning, while the request is still in context.
    let model = leptos_actix::extract::<actix_web::web::Data<mistralrs::Model>>()
        .await?
        .into_inner();

    let (out_tx, out_rx) = tokio::sync::mpsc::channel::<ChatEvent>(64);

    // Turns run one at a time so two messages cannot interleave on this socket.
    tokio::spawn(async move {
        let mut input: std::pin::Pin<
            Box<dyn futures::Stream<Item = Result<String, ServerFnError>> + Send>,
        > = input.into();
        let mut history = initial_history();
        while let Some(message) = input.next().await {
            match message {
                Ok(user_message) => {
                    stream_reply(&model, &mut history, &user_message, &out_tx).await;
                    if out_tx.send(ChatEvent::Done).await.is_err() {
                        break;
                    }
                }
                Err(err) => {
                    log::error!("chat input closed: {err}");
                    let _ = out_tx
                        .send(ChatEvent::Token(format!("\n[chat input error: {err}]")))
                        .await;
                    let _ = out_tx.send(ChatEvent::Done).await;
                    break;
                }
            }
        }
    });

    let output = futures::stream::unfold(out_rx, |mut rx| async move {
        rx.recv()
            .await
            .map(|event| (Ok::<ChatEvent, ServerFnError>(event), rx))
    });
    Ok(output.into())
}

#[cfg(feature = "ssr")]
const SYSTEM_PROMPT: &str = "A chat between a human and an assistant. The assistant gives helpful, detailed, and polite answers.";

#[cfg(feature = "ssr")]
fn initial_history() -> mistralrs::RequestBuilder {
    use mistralrs::{RequestBuilder, TextMessageRole};

    RequestBuilder::new()
        .add_message(TextMessageRole::System, SYSTEM_PROMPT)
        .add_message(
            TextMessageRole::Assistant,
            "Hello - How may I help you today?",
        )
        .add_message(TextMessageRole::User, "What is the capital of France?")
        .add_message(
            TextMessageRole::Assistant,
            "Paris is the capital of France.",
        )
        .set_sampler_temperature(0.8)
        .set_sampler_topp(0.95)
        // Qwen3 closes the think block and answers directly.
        .enable_thinking(false)
}

/// Stream one assistant turn and append it to `history`.
///
/// The engine is shared, so each socket keeps its own message list and
/// sends the full conversation on every turn.
#[cfg(feature = "ssr")]
async fn stream_reply(
    model: &mistralrs::Model,
    history: &mut mistralrs::RequestBuilder,
    user_message: &str,
    out_tx: &tokio::sync::mpsc::Sender<ChatEvent>,
) {
    use mistralrs::{Response, TextMessageRole};

    *history = history
        .clone()
        .add_message(TextMessageRole::User, user_message);

    let mut stream = match model.stream_chat_request(history.clone()).await {
        Ok(stream) => stream,
        Err(err) => {
            log::error!("failed to start inference: {err}");
            let _ = out_tx
                .send(ChatEvent::Token(format!(
                    "\n[failed to start inference: {err}]"
                )))
                .await;
            return;
        }
    };

    let mut assistant = String::new();
    while let Some(response) = stream.next().await {
        match response {
            Response::Chunk(chunk) => {
                let Some(text) = chunk
                    .choices
                    .first()
                    .and_then(|choice| choice.delta.content.clone())
                else {
                    continue;
                };
                if text.is_empty() {
                    continue;
                }
                assistant.push_str(&text);
                if out_tx.send(ChatEvent::Token(text)).await.is_err() {
                    break;
                }
            }
            Response::Done(done) => {
                if assistant.is_empty() {
                    if let Some(text) = done
                        .choices
                        .first()
                        .and_then(|choice| choice.message.content.clone())
                    {
                        assistant = text.clone();
                        let _ = out_tx.send(ChatEvent::Token(text)).await;
                    }
                }
                break;
            }
            Response::ModelError(err, _) => {
                log::error!("model error: {err}");
                let _ = out_tx
                    .send(ChatEvent::Token(format!("\n[model error: {err}]")))
                    .await;
                break;
            }
            Response::InternalError(err) => {
                log::error!("inference error: {err}");
                let _ = out_tx
                    .send(ChatEvent::Token(format!("\n[inference error: {err}]")))
                    .await;
                break;
            }
            Response::ValidationError(err) => {
                log::error!("request rejected: {err}");
                let _ = out_tx
                    .send(ChatEvent::Token(format!("\n[request error: {err}]")))
                    .await;
                break;
            }
            _ => {}
        }
    }

    *history = history
        .clone()
        .add_message(TextMessageRole::Assistant, assistant);
}
