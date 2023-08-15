use leptos::{*, html::Input};

const TYPE_AREA_CLASS: &str = "h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t";
const TYPE_AREA_CLASS_LIGHT: &str = "bg-white border-gray-300";
const TYPE_AREA_CLASS_DARK: &str = "bg-zinc-900 border-zinc-700";

const TEXT_AREA_CLASS: &str = "w-2/3 p-4 border rounded-full input-field";
const TEXT_AREA_CLASS_LIGHT: &str = "border-gray-300 text-black";
const TEXT_AREA_CLASS_DARK: &str = "bg-zinc-700 border-zinc-700 text-white";

const BUTTON_CLASS: &str = "h-full p-4 rounded-full cursor-pointer" ;
const BUTTON_CLASS_LIGHT: &str = "bg-blue-500 text-white";
const BUTTON_CLASS_DARK: &str = "bg-green-700 text-white";

#[component]
pub fn TypeArea(cx: Scope, send: Action<String, Result<(), ServerFnError>>) -> impl IntoView {
    let dark_mode = use_context::<ReadSignal<bool>>(cx).expect("should be able to get dark mode state");

    let type_area_class = Signal::derive(cx, move || {
      if dark_mode.get() {
        format!("{TYPE_AREA_CLASS} {TYPE_AREA_CLASS_DARK}")
      } else {
        format!("{TYPE_AREA_CLASS} {TYPE_AREA_CLASS_LIGHT}")
      }
    });

    let text_area_class = Signal::derive(cx, move || {
      if dark_mode.get() {
        format!("{TEXT_AREA_CLASS} {TEXT_AREA_CLASS_DARK}")
      } else {
        format!("{TEXT_AREA_CLASS} {TEXT_AREA_CLASS_LIGHT}")
      }
    });

    let button_class = Signal::derive(cx, move || {
      if dark_mode.get() {
        format!("{BUTTON_CLASS} {BUTTON_CLASS_DARK}")
      } else {
        format!("{BUTTON_CLASS} {BUTTON_CLASS_LIGHT}")
      }
    });

    let input_ref = create_node_ref::<Input>(cx);
    view!{ cx,
        <div class={type_area_class.get()}>
           <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");
           }
           >
                <input class={text_area_class.get()} type="text" placeholder="Enter your prompt" node_ref=input_ref/>
                <button class={button_class.get()} type="submit">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12h15m0 0l-6.75-6.75M19.5 12l-6.75 6.75" />
                    </svg>
                </button>
           </form>
        </div>
    }
}
