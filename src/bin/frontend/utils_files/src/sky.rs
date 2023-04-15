use getrandom::getrandom;
use wasm_bindgen::JsValue;
use yew::prelude::*;

pub struct Stars {
    stars: Vec<(u8, u8)>,
}

#[derive(Properties, PartialEq)]
pub struct StarsProperties {
    pub max_stars: usize,
    pub star_size: usize,
    pub log: bool,
    // max_clouds: u8,
}

impl Component for Stars {
    type Message = ();
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
    clouds: Vec<u8>,
}

#[derive(Properties, PartialEq)]
pub struct CloudsProperties {
    pub max_clouds: usize,
    pub log: bool,
}

impl Component for Clouds {
    type Message = ();
    type Properties = CloudsProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut random_data: Vec<u8> = (0.._ctx.props().max_clouds)
            .map(|_| 0u8)
            .collect::<Vec<u8>>();
        getrandom(&mut random_data).unwrap_or_else(|error| {
            web_sys::console::log_1(&JsValue::from(format!(
                "sky.rs: create(): getrandom failed to perform byte randomization; {}",
                error
            )));
        });
        if _ctx.props().log {
            web_sys::console::log_1(&JsValue::from(format!(
                "sky.rs: create(): byte generation {:?}",
                random_data
            )));
        }
        let clouds: Vec<u8> = random_data
            .into_iter()
            .map(|item: u8| item % 90)
            .collect::<Vec<u8>>();
        Self { clouds }
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
                    self.clouds
                        .iter()
                        .map(|y_pos: &u8| html! {
                            <div height={format!("{y_pos}%")}>
                                <rect width="50%" height="25%" rx="10" x="10%" y="50%" style="fill: darkgrey; fill-opacity: 0.3;"/>
                                <rect width="50%" height="25%" rx="10" x="15%" y="35%" style="fill: darkgrey; fill-opacity: 0.3;"/>
                                <rect width="50%" height="25%" rx="10" x="25%" y="45%" style="fill: darkgrey; fill-opacity: 0.3;"/>
                                <rect width="50%" height="20%" rx="10" x="20%" y="45%" style="fill: darkgrey; fill-opacity: 0.3;"/>
                                <rect width="30%" height="20%" rx="10" x="30%" y="45%" style="fill: darkgrey; fill-opacity: 0.3;"/>
                                <rect width="30%" height="20%" rx="10" x="30%" y="40%" style="fill: darkgrey; fill-opacity: 0.3;"/>
                                <rect width="30%" height="20%" rx="10" x="35%" y="45%" style="fill: darkgrey; fill-opacity: 0.3;"/>
                            </div>
                        })
                        .collect::<Html>()
                }
            </svg>
        }
    }
}
