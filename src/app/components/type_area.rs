use std::rc::Rc;

use leptos::html::Textarea;
use leptos::prelude::*;

#[component]
pub fn TypeArea(send: UnsyncCallback<String>, busy: Signal<bool>) -> impl IntoView {
    let (empty, set_empty) = signal(true);
    let composer = NodeRef::<Textarea>::new();
    let send_message = Rc::new(move || {
        if busy.get_untracked() {
            return;
        }
        let Some(input) = composer.get() else {
            return;
        };
        let text = input.value();
        if text.trim().is_empty() {
            return;
        }
        send.run(text);
        input.set_value("");
        set_empty.set(true);
    });
    let send_on_key = send_message.clone();

    view! {
        <form class="border-t border-zinc-700 py-4" on:submit=move |ev| {
            ev.prevent_default();
            send_message();
        }>
            <label class="sr-only" for="composer">"Message"</label>
            <div class="rounded-3xl bg-zinc-700 p-2 shadow-sm ring-1 ring-zinc-600 focus-within:ring-blue-500">
                <textarea
                    id="composer"
                    rows="3"
                    maxlength="8000"
                    placeholder="Write a message"
                    class="block w-full resize-none bg-transparent px-3 py-2 text-base text-white outline-none placeholder:text-zinc-400"
                    node_ref=composer
                    on:input=move |ev| {
                        set_empty.set(event_target_value(&ev).trim().is_empty());
                    }
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" && !ev.shift_key() && !ev.is_composing() {
                            ev.prevent_default();
                            send_on_key();
                        }
                    }
                ></textarea>
                <div class="flex items-center justify-between gap-3 px-2 pb-1">
                    <p class="text-xs text-zinc-400">"Enter to send · Shift+Enter for a new line"</p>
                    <button
                        type="submit"
                        class="rounded-full bg-green-700 px-4 py-1.5 text-sm font-semibold text-white hover:bg-green-600 disabled:cursor-not-allowed disabled:bg-zinc-600 disabled:text-zinc-400"
                        disabled=move || busy.get() || empty.get()
                    >
                        {move || if busy.get() { "Sending…" } else { "Send" }}
                    </button>
                </div>
            </div>
        </form>
    }
}
