use web_sys::Window;
use yew::{html, Component, Context, Properties, Callback};

const DONATION_MESSAGE: &str =
    "Although I am not accepting donations right now, just know that I respect and appreciate your consideration.\n\n\n - MXCR_cpu -";
const GITHUB_LINK: &str = "https://github.com/MXCR-cpu/OnDeck";
const INFORMATION: &str =
    "Personal Website as well as explanation of tech stack will be made available in the future";

pub struct Navbar {}

#[derive(Clone)]
pub enum NavbarMsg {
    GoTo(String),
    Alert(String),
}

#[derive(Properties, PartialEq)]
pub struct NavbarProp {
    pub window: Window,
    pub day: bool,
    pub settings: bool,
    pub change_day: Callback<web_sys::MouseEvent>,
    pub change_setting: Callback<web_sys::MouseEvent>,
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = NavbarProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::GoTo(link) => {
                ctx.props()
                    .window
                    .location()
                    .set_href(link.as_str())
                    .unwrap();
            }
            Self::Message::Alert(message) => {
                ctx.props()
                    .window
                    .alert_with_message(message.as_str())
                    .unwrap();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let onclick = |message: Self::Message| ctx.link().callback(move |_| message.clone());
        html! {
            <div class={"top_row"}>
                <button class={"button_col_0"} onclick={onclick(Self::Message::Alert(DONATION_MESSAGE.to_string()))} alt={"Donations"}>
                    { "💸" }
                </button>
                <button class={"button_col_1"} onclick={onclick(Self::Message::GoTo(GITHUB_LINK.to_string()))}>
                    { "🐙" }
                </button>
                <button class={"button_col_3"} onclick={onclick(Self::Message::Alert(INFORMATION.to_string()))}>
                    { "🧠" }
                </button>
                <button class={"button_col_4"} onclick={ctx.props().change_day.clone()}>{
                    if ctx.props().day { "☀️" } else { "🌙" }
                }</button>
                <button class={"button_col_5"} onclick={ctx.props().change_setting.clone()}>{
                    if ctx.props().settings { "🚀" } else { "⚙️" }
                }</button>
            </div>
        }
    }
}
