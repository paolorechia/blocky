use log::info;
use yew::prelude::*;

#[function_component]
pub fn Block() -> Html {

    let keypress = {
        Callback::from(move |e: KeyboardEvent| {
            info!("Key {:?}", e.key());
            info!("Code {:?}", e.code());
            info!("Alt {:?}", e.alt_key());
            info!("Ctrl {:?}", e.ctrl_key());
            info!("Shift {:?}", e.shift_key());
            info!("Meta {:?}", e.meta_key());

        })
    };
    info!(" Hello??");

    html! {
        <main>
            <textarea onkeydown={keypress.clone()}></textarea>
        </main>
    }
}
