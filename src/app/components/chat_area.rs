use leptos::{*, html::Div};

use crate::model::conversation::Conversation;

fn conversation_to_nodes(cx: Scope, conversation: Conversation) -> impl IntoView {
    view!{cx, <>
          {move || conversation.messages.iter().map(move |message| {
                    if !message.user {
                        view! {cx,
                            <div class="max-w-md p-4 mb-5 rounded-lg self-start bg-gray-200 text-black">
                               {message.text.clone()}
                            </div>
                        }
                    } else {
                        view! {cx,
                            <div class="max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white">
                                {message.text.clone()}
                            </div>
                        }
                    }
                }
                ).collect::<Vec<_>>()
          }
          </>
    }
}

#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>(cx);
    create_effect(cx, move |_| {
      dbg!(conversation());
      if let Some(div) = chat_div_ref.get() {
        div.set_scroll_top(div.scroll_height());
      }
    });
    view!{ cx,

        <div class="h-screen pb-24 w-full flex flex-col overflow-y-auto border border-gray-300 rounded p-5 bg-gray-100" node_ref=chat_div_ref>
           {move || conversation_to_nodes(cx, conversation.get())}
        </div>
    }
}
