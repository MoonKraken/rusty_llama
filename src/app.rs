use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;

#[cfg(target_arch = "wasm32")]
use crate::api::ChatEvent;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rusty_llama.css"/>
        <Title text="Rusty Llama"/>
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=StaticSegment("") view=ChatPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn ChatPage() -> impl IntoView {
    let (conversation, set_conversation) = signal(Conversation::new());
    let busy = Signal::derive(move || {
        conversation.with(|conversation| {
            conversation
                .messages
                .last()
                .is_some_and(|message| !message.user && message.pending)
        })
    });

    // The browser build opens the socket. SSR renders the page and does not.
    #[cfg(target_arch = "wasm32")]
    let outgoing_tx = {
        use futures::StreamExt;
        use leptos::server_fn::error::ServerFnError;
        use std::cell::RefCell;
        use std::rc::Rc;

        let (outgoing_tx, outgoing_rx) = futures::channel::mpsc::unbounded();
        let outgoing_rx = Rc::new(RefCell::new(Some(outgoing_rx)));
        Effect::new(move |_| {
            let Some(outgoing_rx) = outgoing_rx.borrow_mut().take() else {
                return;
            };
            leptos::task::spawn_local(async move {
                let input = outgoing_rx.map(|text| Ok::<_, ServerFnError>(text));
                let mut incoming = match crate::api::chat(input.into()).await {
                    Ok(incoming) => incoming,
                    Err(err) => {
                        leptos::logging::error!("chat socket: {err}");
                        set_conversation.update(|conversation| {
                            if let Some(last) = conversation
                                .messages
                                .last_mut()
                                .filter(|message| !message.user)
                            {
                                last.text.push_str(&format!("\n[connection error: {err}]"));
                                last.pending = false;
                            }
                        });
                        return;
                    }
                };

                while let Some(message) = incoming.next().await {
                    match message {
                        Ok(ChatEvent::Token(token)) => {
                            set_conversation.update(|conversation| {
                                if let Some(last) = conversation.messages.last_mut() {
                                    last.text.push_str(&token);
                                }
                            });
                        }
                        Ok(ChatEvent::Done) => {
                            set_conversation.update(|conversation| {
                                if let Some(last) = conversation.messages.last_mut() {
                                    last.pending = false;
                                }
                            });
                        }
                        Err(err) => {
                            leptos::logging::error!("chat stream: {err}");
                            set_conversation.update(|conversation| {
                                if let Some(last) = conversation.messages.last_mut() {
                                    last.text.push_str(&format!("\n[stream error: {err}]"));
                                    last.pending = false;
                                }
                            });
                            break;
                        }
                    }
                }
            });
        });
        outgoing_tx
    };

    let send = UnsyncCallback::new(move |new_message: String| {
        if new_message.trim().is_empty() {
            return;
        }
        set_conversation.update(|conversation| {
            conversation.messages.push(Message {
                text: new_message.clone(),
                user: true,
                pending: false,
            });
            conversation.messages.push(Message {
                text: String::new(),
                user: false,
                pending: true,
            });
        });

        #[cfg(target_arch = "wasm32")]
        if outgoing_tx.unbounded_send(new_message).is_err() {
            leptos::logging::error!("chat socket is closed");
            set_conversation.update(|conversation| {
                if let Some(last) = conversation.messages.last_mut() {
                    last.pending = false;
                    last.text.push_str("\n[chat socket is closed]");
                }
            });
        }
    });

    view! {
        <main class="mx-auto flex h-full w-full max-w-3xl flex-col px-4 sm:px-6">
            <ChatArea conversation/>
            <TypeArea send busy/>
        </main>
    }
}
