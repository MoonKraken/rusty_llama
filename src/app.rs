use std::cell::RefCell;
use std::rc::Rc;

use futures::stream::SplitSink;
use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;

use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // allow any component to get dark mode state via context
    let (dark_mode, _) = create_signal(true);
    provide_context(dark_mode);

    let (conversation, set_conversation) = create_signal(Conversation::new());

    use gloo_net::websocket::futures::WebSocket;
    use gloo_net::websocket::Message::Text as Txt;
    use futures::{SinkExt, StreamExt};
    let client: Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>>
        = Default::default();

    let client_clone_baby = client.clone();
    create_effect(move |_| {
        let location = web_sys::window().unwrap().location();
        let hostname = location.hostname().expect("failed to retrieve origin hostname");
        let ws_url = format!("ws://{hostname}:3000/ws");

        let connection = WebSocket::open(&format!("{ws_url}")).expect("failed to establish WebSocket connection");

        let (sender, mut recv) = connection.split();
        spawn_local(async move {
            while let Some(msg) = recv.next().await {
                match msg {
                    Ok(Txt(msg)) => {
                        set_conversation.update(move |c| {
                            c.messages.last_mut().unwrap().text.push_str(&msg);
                        });
                    }
                    _ => { break; }
                }
            }
        });

        *client_clone_baby.borrow_mut() = Some(sender);
    });

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        let client2 = client.clone();
        let msg = new_message.to_string();
        async move {
            client2
                .borrow_mut()
                .as_mut()
                .unwrap()
                .send(Txt(msg.to_string()))
                .await
                .map_err(|_| ServerFnError::ServerError("WebSocket issue".to_string()))
        }
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::new(),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/rusty_llama.css"/>

        // sets the document title
        <Title text="Rusty Llama"/>
        <ChatArea conversation/>
        <TypeArea send/>
    }
}
