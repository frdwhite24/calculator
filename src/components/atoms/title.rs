use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(PartialEq)]
pub enum Level {
    One,
    Two,
}

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub text: String,
    pub level: Level,
}

#[styled_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let stylesheet = style!(
        r#"
            border: 1px solid red;
        "#
    )
    .unwrap();
    html! {
        if &props.level == &Level::One {
            <h1 class={stylesheet}>{&props.text}</h1>
        } else if &props.level == &Level::Two {
            <h2 class={stylesheet}>{&props.text}</h2>
        }
    }
}
