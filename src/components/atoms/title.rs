use yew::prelude::*;

#[derive(PartialEq)]
pub enum Level {
    One,
    Two,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub level: Level,
}

#[function_component(Title)]
pub fn title(props: &Props) -> Html {
    html! {
        if &props.level == &Level::One {
            <h1>{&props.text}</h1>
        } else if &props.level == &Level::Two {
            <h2>{&props.text}</h2>
        }
    }
}
