use std::cell::Cell;
use std::rc::Rc;

use leptos::html::Div;
use leptos::prelude::*;

use crate::model::conversation::Conversation;

const USER_MESSAGE_CLASS: &str =
    "ml-auto max-w-[85%] rounded-2xl rounded-br-md bg-blue-500 px-4 py-3 text-white";
const ASSISTANT_MESSAGE_CLASS: &str =
    "mr-auto max-w-[85%] rounded-2xl rounded-bl-md bg-zinc-700 px-4 py-3 text-white";

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = NodeRef::<Div>::new();
    // Stay pinned to the latest line unless the reader has scrolled up.
    let follow = Rc::new(Cell::new(true));
    let follow_on_scroll = follow.clone();

    Effect::new(move |_| {
        conversation.track();
        if !follow.get() {
            return;
        }
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        <div
            id="transcript"
            role="log"
            aria-live="polite"
            aria-relevant="additions"
            class="min-h-0 flex-1 overflow-y-auto"
            node_ref=chat_div_ref
            on:scroll=move |_| {
                let Some(div) = chat_div_ref.get() else {
                    return;
                };
                let gap = div.scroll_height() - div.scroll_top() - div.client_height();
                follow_on_scroll.set(gap < 80);
            }
        >
            <div class="flex min-h-full flex-col gap-5 py-6">
                {move || {
                    let messages = conversation.get().messages;
                    if messages.is_empty() {
                        view! {
                            <div class="m-auto max-w-md px-2 text-center">
                                <p class="text-xl font-semibold text-zinc-100">
                                    "Welcome to Rusty Llama!"
                                </p>
                                <p class="mt-2 text-sm leading-6 text-zinc-400">
                                    "Write a prompt in the message field below!"
                                </p>
                            </div>
                        }
                        .into_any()
                    } else {
                        messages
                            .into_iter()
                            .map(|message| {
                                let class_name = if message.user {
                                    USER_MESSAGE_CLASS
                                } else {
                                    ASSISTANT_MESSAGE_CLASS
                                };
                                let label = if message.user { "You" } else { "Assistant" };
                                let show_dots = !message.user && message.text.is_empty();
                                let show_caret =
                                    message.pending && !message.user && !message.text.is_empty();
                                let html = crate::markdown::render(&message.text);
                                view! {
                                    <article class=class_name>
                                        <p class="mb-1 text-[0.7rem] font-semibold uppercase tracking-wider opacity-70">
                                            {label}
                                        </p>
                                        {if show_dots {
                                            view! {
                                                <p class="flex items-center gap-1 py-1" aria-label="The assistant is writing">
                                                    <span class="size-1.5 animate-bounce rounded-full bg-current [animation-delay:0ms]"></span>
                                                    <span class="size-1.5 animate-bounce rounded-full bg-current [animation-delay:150ms]"></span>
                                                    <span class="size-1.5 animate-bounce rounded-full bg-current [animation-delay:300ms]"></span>
                                                </p>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div>
                                                    <div class="md" inner_html=html></div>
                                                    {show_caret.then(|| view! {
                                                        <span class="mt-1 inline-block h-4 w-px animate-pulse bg-current align-middle"></span>
                                                    })}
                                                </div>
                                            }.into_any()
                                        }}
                                    </article>
                                }
                            })
                            .collect_view()
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}
