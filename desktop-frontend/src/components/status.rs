use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Status {
    props: StatusProp
}

#[derive(Properties)]
pub struct StatusProp {
    #[props(required)]
    pub gba: Rc<RefCell<GBA>>
}

pub enum Msg {}

impl Component for Status {
    type Message = Msg;
    type Properties = StatusProp;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Status {
            props: props
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl Renderable<Status> for Status {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h4>{"Status"}</h4>
                <p>{format!("Instruction Set: {:?}", self.props.gba.borrow().cpu.current_instruction_set)}</p>
                <p>{format!("Operating Mode: {:?}", self.props.gba.borrow().cpu.operating_mode)}</p>
            </div>
        }
    }
}