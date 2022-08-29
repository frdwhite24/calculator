use yew::prelude::*;

mod components;
use components::atoms::key::{Key, KeyType};
use components::atoms::title::{Level, Title};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Title text="Calculator" level={Level::One}/>
            <Title text="by Fred White" level={Level::Two}/>
            <Key text="1" key_type={KeyType::Tall}/>
        </>
    }
}
