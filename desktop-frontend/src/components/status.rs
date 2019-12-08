use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::InstructionSet;
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Status {
            props: props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            <div class="card row">
                <h3>{"Status"}</h3>
                <p>{format!("Instruction Set: {:?}", self.props.gba.borrow().cpu.current_instruction_set)}</p>
                <p>{format!("Operating Mode: {:?}", self.props.gba.borrow().cpu.operating_mode)}</p>
                <p>{&self.props.gba.borrow().cpu.last_instruction}</p>
            </div>
        }
    }
}