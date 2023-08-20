use yew::prelude::*;
use trunk_template::components::hello_world::HelloWorld;
use trunk_template::components::block::Block;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <HelloWorld />
            <Block />
        </main>
    }
}
