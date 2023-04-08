use getrandom::getrandom;
use wasm_bindgen::JsValue;
use yew::prelude::*;

pub struct Sky {
    stars: Vec<(u8, u8)>,
    // clouds: Vec<(u8, u8)>,
}

#[derive(Properties, PartialEq)]
pub struct SkyProperties {
    pub max_stars: usize,
    pub star_size: usize,
    pub log: bool,
    // max_clouds: u8,
}

impl Component for Sky {
    type Message = ();
    type Properties = SkyProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut random_data: Vec<u8> = (0.._ctx.props().max_stars)
            .map(|_| 0u8)
            .collect::<Vec<u8>>();
        getrandom(&mut random_data).unwrap_or_else(|error| {
            web_sys::console::log_1(&JsValue::from(format!("sky.rs: create(): getrandom failed to perform byte randomization; {}", error)));
        });
        if _ctx.props().log {
            web_sys::console::log_1(&JsValue::from(format!("sky.rs: create(): byte generation {:?}", random_data)));
        }
        let stars: Vec<(u8, u8)> = (0..((random_data.len() / 2) as usize))
            .map(|index: usize| (random_data[2 * index] % 100, random_data[(2 * index) + 1] % 100))
            .collect::<Vec<(u8, u8)>>();
        // let clouds: Vec<(u8, u8)> = (0.._ctx.props().max_clouds)
        //     .map(|_| (rng.gen::<u8>(), rng.gen::<u8>()))
        //     .collect::<Vec<(u8, u8)>>();
        Self {
            stars,
            // clouds,
        }
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
