use log::info;
use yew::prelude::*;
use web_sys::HtmlTextAreaElement;

#[function_component]
pub fn Block() -> Html {
    let state_string: UseStateHandle<String> = use_state(|| String::from(""));
    let command_buffer: UseStateHandle<String> = use_state(|| String::from(""));
    let is_typing_command: UseStateHandle<bool> = use_state(|| false);

    let onkeydown_handler = {
        let new_is_typing_command = is_typing_command.clone();
        let new_command_buffer = command_buffer.clone();
        Callback::from(move | e:KeyboardEvent| {
            // submit command buffer
            if e.code() == "Enter" && *new_is_typing_command == true {
                info!("TODO: Should submit command here!");
                new_is_typing_command.set(false);
                new_command_buffer.set(String::from(""));
            }
            // start command buffer
            if e.code() == "Slash" && *new_is_typing_command == false {
                info!("Detected slash, should check for command");
                new_is_typing_command.set(true);
                e.prevent_default();
                let new_command = format!("{}{}", *new_command_buffer, e.key());
                let new_command = new_command.to_string();
                new_command_buffer.set(new_command);
                return
            }
            // handle command buffer
            if *new_is_typing_command == true {
                e.prevent_default();
                if e.code() == "Escape" {
                    info!("Detected escape, should cancel command");
                    new_is_typing_command.set(false);
                    new_command_buffer.set(String::from(""));
                }
                else if e.code() == "Backspace" {
                    let mut new_command = format!("{}", *new_command_buffer).to_string();
                    new_command.pop();

                    if new_command.len() == 0 {
                        new_is_typing_command.set(false);
                        new_command_buffer.set(String::from(""));
                    } else {
                        new_command_buffer.set(new_command);
                    }
                }
                else if (e.which() >= 48 && e.which() <= 57) || (e.which() >= 65 && e.which() <= 90) {
                    // Digits or lower case letters or slash
                    info!("Detected letter/digit");
                    let new_command = format!("{}{}", *new_command_buffer, e.key());
                    let new_command = new_command.to_string();
                    new_command_buffer.set(new_command);
                }
            }
        })
    };

    let onkeyup_handler = {
        let new_state_string = state_string.clone();
        let new_command_buffer = command_buffer.clone();
        let new_is_typing_command = is_typing_command.clone();

        Callback::from(move | e:KeyboardEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let val = input.value();
            new_state_string.set(val);
            info!("{:?}", *new_command_buffer);
            info!("{:?}", *new_is_typing_command);
        })
    };
    let string_copy = (*state_string).clone();
    let command_buffer_copy = (*command_buffer).clone();
    let is_typing_command_copy = (*is_typing_command).clone();
    html! {
        <main>
            <textarea onkeydown={onkeydown_handler.clone()} onkeyup={onkeyup_handler.clone()}></textarea>
            <div> {"You've typed: "}{string_copy} </div>
            if is_typing_command_copy {
                <div> {"Your command buffer: "} {command_buffer_copy} </div>
            }
        </main>
    }
}
