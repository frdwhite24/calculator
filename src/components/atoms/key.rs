use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(PartialEq)]
pub enum KeyType {
    Regular,
    Tall,
}

impl KeyType {
    pub fn to_string(&self) -> String {
        match self {
            KeyType::Regular => "regular".to_owned(),
            KeyType::Tall => "tall".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    pub text: String,
    pub key_type: KeyType,
}

#[styled_component(Key)]
pub fn title(props: &KeyProps) -> Html {
    let stylesheet = style!(
        r#"
        .regular {
            border: 1px solid red;
        }

        .tall {
            border: 1px solid green;
        }
        "#
    )
    .unwrap();
    html! {
        <span class={stylesheet}>
            <button class={props.key_type.to_string()}>{&props.text}</button>
        </span>
    }
}
