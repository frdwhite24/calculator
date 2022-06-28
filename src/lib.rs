use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

mod components;

use components::atoms::title::{Level, Title};

#[derive(Serialize, Deserialize)]
struct MyData {
    username: String,
    age: i64,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "Tester";
    let data = MyData {
        username: name.to_owned(),
        age: 74,
    };

    // NOTE: example of logging using gloo and serde
    log!(name);
    log!(serde_json::to_string_pretty(&data).unwrap());

    html! {
        <>
            <Title text="Calculator" level={Level::One}/>
            <Title text="by Fred White" level={Level::Two}/>
        </>
    }
}
