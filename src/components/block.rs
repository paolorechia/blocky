use log::info;
use yew::prelude::*;
use web_sys::HtmlTextAreaElement;

#[function_component]
pub fn Block() -> Html {
    let state_string: UseStateHandle<String> = use_state(|| String::from(""));
    let command_buffer: UseStateHandle<String> = use_state(|| String::from(""));
    let is_typing_command: UseStateHandle<bool> = use_state(|| false);

    let onchange_handler = {
        let new_state_string = state_string.clone();
        let new_command_buffer = command_buffer.clone();
        let new_is_typing_command = is_typing_command.clone();

        Callback::from(move | e:KeyboardEvent| {
            if e.code() == "Slash" {
                info!("Detected slash, should check for command");
                is_typing_command.set(true);
            }
            if e.code() == "Escape" {
                info!("Detected escpae, should cancel command");
                is_typing_command.set(false);
            }
            if *is_typing_command == true {
                // Digits or lower case letters
                if (e.which() >= 48 && e.which() <= 57) || (e.which() >= 65 && e.which() <= 90) {
                    info!("Detected letter/digit");
                    let new_command = format!("{}{}", *new_command_buffer, e.key());
                    let new_command = new_command.to_string();
                    new_command_buffer.set(new_command);
                }
            } else {
                let input: HtmlTextAreaElement = e.target_unchecked_into();
                let val = input.value();
                new_state_string.set(val);
                info!("{:?}", *new_command_buffer);
                info!("{:?}", *is_typing_command);
            }
        })
    };
    let string_copy = (*state_string).clone();
    let command_buffer_copy = (*command_buffer).clone();
    html! {
        <main>
            <textarea onkeyup={onchange_handler.clone()}></textarea>
            <div> {"You've typed: "}{string_copy} </div>
            <div> {"Your command buffer: "}{command_buffer_copy} </div>
        </main>
    }
}
