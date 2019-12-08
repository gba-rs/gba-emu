use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct NavBar {}

pub enum Msg {}

impl Component for NavBar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavBar {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<NavBar> for NavBar {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="navbar">
                <a>{"GBA Emulator"}</a>
                <a>{"Debugger"}</a>
            </div>
        }
    }
}