use interact::{
    link::{GameList, GameListEntry},
    site::SITE_LINK,
};
use utils_files::{
    animation_level::AnimationLevel,
    event_source_state::EventSourceState,
    request::{get_request, send_player_amount_update},
    web_error::ClientError,
};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::{classes, html, Callback, Component, Context, Html, NodeRef, Properties};

#[derive(Clone, PartialEq)]
pub enum Pages {
    Main,
    Settings,
}

pub struct Panel {
    player_amount_selection: u8,
    animation_level: AnimationLevel,
    player_id_ref: NodeRef,
    links: Option<GameList>,
    event_source: EventSourceState,
}

#[derive(Clone)]
pub enum PanelMsg {
    AddPlayer,
    SubPlayer,
    SelectAnimationLevel(AnimationLevel),
    ApplySettings,
    Send(u8),
    AwaitUpdate,
    Update(Option<Vec<GameListEntry>>),
    EndUpdate,
    Response(ClientError),
    None,
}

#[derive(Properties, PartialEq)]
pub struct PanelProp {
    pub page_selection: Pages,
    pub player_id_tag: String,
    pub change_player_id: Callback<String>,
    pub change_animation_level: Callback<AnimationLevel>,
    pub reload_page: Callback<()>,
    pub log: bool,
}

impl Component for Panel {
    type Message = PanelMsg;
    type Properties = PanelProp;

    fn create(ctx: &Context<Self>) -> Self {
        let callback_update = ctx.link().callback(move |_: ()| Self::Message::AwaitUpdate);
        let callback_end = ctx.link().callback(move |_: ()| Self::Message::EndUpdate);
        callback_update.emit(());
        let event_source: EventSourceState = EventSourceState::new(
            &format!("{SITE_LINK}/main/page_stream"),
            None,
            move |_| callback_update.emit(()),
            move |_| callback_end.emit(()),
        );
        Self {
            player_amount_selection: 2,
            animation_level: AnimationLevel::High,
            player_id_ref: NodeRef::default(),
            links: None,
            event_source,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::AddPlayer => {
                if self.player_amount_selection < 8 {
                    self.player_amount_selection += 1;
                }
            }
            Self::Message::SubPlayer => {
                if self.player_amount_selection > 2 {
                    self.player_amount_selection -= 1;
                }
            }
            Self::Message::SelectAnimationLevel(animation_level) => {
                if ctx.props().log {
                    ctx.link()
                        .send_message(Self::Message::Response(ClientError::from(
                            file!(),
                            "update(): AnimationLevel Changed",
                        )));
                }
                self.animation_level = animation_level;
            }
            Self::Message::ApplySettings => {
                let new_player_id: String = self
                    .player_id_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                if ctx.props().log {
                    ctx.link()
                        .send_message(Self::Message::Response(ClientError::from(
                            file!(),
                            &format!(
                                "update(): {}, {}",
                                new_player_id,
                                match self.animation_level {
                                    AnimationLevel::High => "High",
                                    AnimationLevel::Low => "Low",
                                    AnimationLevel::None => "None",
                                }
                            ),
                        )));
                }
                if !new_player_id.is_empty() {
                    ctx.props().change_player_id.emit(new_player_id);
                }
                ctx.props()
                    .change_animation_level
                    .emit(self.animation_level.clone());
                ctx.props().reload_page.emit(());
            }
            Self::Message::Send(number_of_players) => {
                ctx.link().send_future(async move {
                    match send_player_amount_update(number_of_players).await {
                        Ok(()) => Self::Message::None,
                        Err(error) => Self::Message::Response(
                            error.push(file!(), "update(): failed to send future"),
                        ),
                    }
                });
            }
            Self::Message::AwaitUpdate => {
                ctx.link().send_future(async move {
                    Self::Message::Update(
                        get_request::<Option<Vec<GameListEntry>>>(
                            format!("{}/active_game_links", SITE_LINK).as_str(),
                        )
                        .await
                        .unwrap_or(None),
                    )
                });
            }
            Self::Message::Update(game_links_option) => {
                self.links = game_links_option;
            }
            Self::Message::EndUpdate => {
                self.event_source.close_connection();
            }
            Self::Message::Response(client_error) => {
                web_sys::console::log_1(&JsValue::from(format!("{}", client_error)));
            }
            _ => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = |message: PanelMsg| ctx.link().callback(move |_| message.clone());
        match ctx.props().page_selection {
            Pages::Main => html! {
                <div class={"panel_base"}>
                    <h2 class={classes!("panel_header", "font")}>{
                        format!("{} Player Free-for-all Battleship", self.player_amount_selection)
                    }</h2>
                    <div class={classes!("menu_screen", "font")}>
                        <button
                            class={classes!("menu_button", "button_col_0")}
                            onclick={onclick(Self::Message::AddPlayer)}>{
                                "Add Player"
                            }</button>
                        <button
                            class={classes!("menu_button", "button_col_1")}
                            onclick={onclick(Self::Message::Send(self.player_amount_selection.clone()))}>{
                                "Start Game"
                            }</button>
                        <button
                            class={classes!("menu_button", "button_col_2")}
                            onclick={onclick(Self::Message::SubPlayer)}>{
                                "Subtract Player"
                            }</button>
                    </div>
                    <div class={classes!("links_base", "font")}>
                        <ul class={"links_holder"}>{
                            match &self.links {
                                Some(item) => item
                                    .iter()
                                    .map(|entry: &GameListEntry| html! {
                                        <li><a class={classes!("links", "font")}
                                            href={format!(
                                                    "{}/game/{}/{}",
                                                    SITE_LINK,
                                                    entry.game_record_number,
                                                    ctx.props().player_id_tag
                                                )}>{
                                                format!("{:#}", entry)
                                            }</a>
                                        </li>
                                    })
                                    .collect::<Html>(),
                                None => html!{
                                    <p class={"font"}>{
                                        "Select the number of players and start the game"
                                    }</p>
                                }
                            }
                        }</ul>
                    </div>
                </div>
            },
            Pages::Settings => html! {
                <div class={"panel_base"}>
                    <h2 class={classes!("panel_header", "font")}>{
                        "Settings"
                    }</h2>
                    <div id="settings_base" class="font">
                        <form action="" id="settings_form">
                            <label
                                for="player_id"
                                class="settings_label">{
                                    "Player Id:"
                                }</label>
                            <input
                                type="text"
                                ref={&self.player_id_ref}
                                id="player_id"
                                class="settings_option"
                                name="fname"
                                placeholder={ctx.props().player_id_tag.clone()} />
                            <br/><br/><br/>
                            <label
                                for="animation_level"
                                class="settings_label">{
                                "Animation Level:"
                            }</label>
                            <select
                                id="animation_level"
                                class="settings_option">
                                <option onclick={onclick(
                                    Self::Message::SelectAnimationLevel(
                                        AnimationLevel::None))}>{
                                    "None"
                                }</option>
                                <option onclick={onclick(
                                    Self::Message::SelectAnimationLevel(
                                        AnimationLevel::Low))}>{
                                    "Low"
                                }</option>
                                <option onclick={onclick(
                                    Self::Message::SelectAnimationLevel(
                                        AnimationLevel::High))}>{
                                    "High"
                                }</option>
                            </select>
                        </form>
                    </div>
                    <div id="settings_apply" class="font">
                        <button onclick={onclick(Self::Message::ApplySettings)}>{
                            "Apply"
                        }</button>
                    </div>
                </div>
            },
        }
    }

    fn destroy(&mut self, ctx: &yew::Context<Self>) {
        ctx.link().send_message(Self::Message::EndUpdate);
    }
}
