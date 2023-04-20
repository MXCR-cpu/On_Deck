use getrandom::getrandom;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::web_error::ClientError;

pub struct Stars {
    stars: Vec<(u8, u8)>,
}

pub enum StarsMsg {
    Response(ClientError),
}

#[derive(Properties, PartialEq)]
pub struct StarsProperties {
    pub max_stars: usize,
    pub star_size: usize,
    pub log: bool,
}

impl Component for Stars {
    type Message = StarsMsg;
    type Properties = StarsProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut random_data: Vec<u8> = (0.._ctx.props().max_stars)
            .map(|_| 0u8)
            .collect::<Vec<u8>>();
        getrandom(&mut random_data).unwrap_or_else(|error| {
            web_sys::console::log_1(&JsValue::from(format!(
                "sky.rs: Stars: create(): getrandom failed to perform byte randomization; {}",
                error
            )));
        });
        if _ctx.props().log {
            web_sys::console::log_1(&JsValue::from(format!(
                "sky.rs: Stars: create(): byte generation {:?}",
                random_data
            )));
        }
        let stars: Vec<(u8, u8)> = (0..((random_data.len() / 2) as usize))
            .map(|index: usize| {
                (
                    random_data[2 * index] % 100,
                    random_data[(2 * index) + 1] % 100,
                )
            })
            .collect::<Vec<(u8, u8)>>();
        Self { stars }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Response(error) => {
                web_sys::console::log_1(&JsValue::from(format!("{}", error)));
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <svg xmlns={"http://www.w3.org/2000/svg"} version={"1.0"}
                width={"100%"} height={"100%"}>
                <defs>
                    <filter id="f1" x="0" y="0">
                        <fegaussianblur in="SourceGraphic" stdDeviation="10" />
                    </filter>
                </defs>
                {
                    self.stars
                        .iter()
                        .map(|(x_pos, y_pos): &(u8, u8)| html! {
                            <circle
                                cx={format!("{}%", x_pos.to_string())}
                                cy={format!("{}%", y_pos.to_string())}
                                r={_ctx.props().star_size.to_string()}
                                fill={"yellow"}
                                fill-opacity={(0.5).to_string()}/>
                        })
                        .collect::<Html>()
                }
            </svg>
        }
    }
}

pub struct Clouds {
    clouds: Vec<Vec<(u8, u8)>>,
}

pub enum CloudsMsg {
    Response(ClientError),
}

#[derive(Properties, PartialEq)]
pub struct CloudsProperties {
    pub max_clouds: usize,
    pub log: bool,
    pub day: bool,
}

impl Component for Clouds {
    type Message = CloudsMsg;
    type Properties = CloudsProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let mut random_data: Vec<u8> = (0..ctx.props().max_clouds * 14)
            .map(|_| 0u8)
            .collect::<Vec<u8>>();
        getrandom(&mut random_data).unwrap_or_else(|error| {
            ctx.link()
                .send_message(Self::Message::Response(ClientError::from(
                    file!(),
                    &format!(
                        "create(): getrandom failed to perform byte randomization: {}",
                        error
                    ),
                )));
        });
        if ctx.props().log {
            ctx.link()
                .send_message(Self::Message::Response(ClientError::from(
                    file!(),
                    &format!("create(): byte generation: {:?}", random_data),
                )));
        }
        let clouds: Vec<Vec<(u8, u8)>> = (0..ctx.props().max_clouds)
            .map(|scalar: usize| {
                (0..7)
                    .map(|inner_scalar: usize| {
                        (
                            random_data[(7 * scalar) + inner_scalar] % 50,
                            random_data[(7 * scalar) + inner_scalar + 7] % 75,
                        )
                    })
                    .collect::<Vec<(u8, u8)>>()
            })
            .collect::<Vec<Vec<(u8, u8)>>>();
        Self { clouds }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Response(error) => {
                web_sys::console::log_1(&JsValue::from(format!("{}", error)));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg
                width="100%"
                height="100%"
                version="1.0">
                <defs>
                    <filter id="f1" x="0" y="0">
                        <fegaussianblur in="SourceGraphic" stdDeviation="10" />
                    </filter>
                </defs>
                {
                    self.clouds
                        .iter()
                        .map(|values: &Vec<(u8,u8)>| html! {
                            <svg
                                x={format!("{}%", values[0].0)}
                                y={format!("{}%", values[0].1)}
                                width="300"
                                height="150"
                                version="1.0">{
                                values
                                    .iter()
                                    .map(|(value_1, value_2): &(u8, u8)| html! {
                                        <rect
                                            width="50%"
                                            height="25%"
                                            rx="10"
                                            x={format!("{}%", value_1)}
                                            y={format!("{}%", value_2)}
                                            style={
                                                if ctx.props().day {
                                                    "
                                                    fill: lightgrey;
                                                    fill-opacity: 0.5;
                                                    overflow: visible;
                                                    "
                                                } else {
                                                    "
                                                    fill: darkgrey;
                                                    fill-opacity: 0.1;
                                                    overflow: visible;
                                                    "
                                                }
                                            }/>
                                    })
                                .collect::<Html>()
                            }</svg>
                        })
                        .collect::<Html>()
                }
            </svg>
        }
    }
}
