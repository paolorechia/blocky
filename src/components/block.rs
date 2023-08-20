use log::info;
use yew::prelude::*;
use web_sys::HtmlTextAreaElement;

#[function_component]
pub fn Block() -> Html {
    let state_string: UseStateHandle<String> = use_state(|| String::from(""));

    let onchange_handler = {
        let new_state_string = state_string.clone();

        Callback::from(move | e:InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let val = input.value();
            new_state_string.set(val);
        })
    };
    let string_copy = (*state_string).clone();
    html! {
        <main>
            <textarea oninput={onchange_handler.clone()}></textarea>
            <div> {"You've typed: "}{string_copy} </div>
        </main>
    }
}
