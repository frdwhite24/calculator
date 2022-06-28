use yew::{html, Component, Context, Html, Properties};

pub struct IncrementButton;

impl Component for IncrementButton {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <button>{"+2"}</button>
        }
        // <button>{&ctx.props().button_text}</button>
    }
}
