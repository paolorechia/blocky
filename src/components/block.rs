use log::info;
use yew::prelude::*;

#[function_component]
pub fn Block() -> Html {

    let caret = use_state(|| 0);
    let state_string: UseStateHandle<String> = use_state(|| String::from(""));

    let keypress = {
        let caret = caret.clone();
        let new_state_string = state_string.clone();
        Callback::from(move | e: KeyboardEvent| {
            info!("Key {:?}", e.key());
            info!("Code {:?}", e.code());
            info!("Alt {:?}", e.alt_key());
            info!("Ctrl {:?}", e.ctrl_key());
            info!("Shift {:?}", e.shift_key());
            info!("Meta {:?}", e.meta_key());
            let formatted_str = format!("{}{}", *(new_state_string.clone()), e.key());
            let string = formatted_str.to_string();
            caret.set(*caret + 1);
            new_state_string.set(string);
        })
    };
    info!(" Hello??");
    let string_copy = (*state_string).clone();
    html! {
        <main>
            <textarea onkeydown={keypress.clone()}></textarea>
            <div>{"Your caret is "}{*caret} </div>
            <div> {"You've typed: "}{string_copy} </div>
        </main>
    }
}
