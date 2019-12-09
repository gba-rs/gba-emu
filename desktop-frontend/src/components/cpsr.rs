use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Cpsr {
    props: CpsrProp
}

#[derive(Properties)]
pub struct CpsrProp {
    #[props(required)]
    pub gba: Rc<RefCell<GBA>>
}

pub enum Msg {}

impl Component for Cpsr {
    type Message = Msg;
    type Properties = CpsrProp;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Cpsr {
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

impl Renderable<Cpsr> for Cpsr {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="card row">
                <h3>{"Current Program Status Register"}</h3>
                <ul>
                    <li>{format!("Carry: {}", self.props.gba.borrow().cpu.cpsr.flags.carry)}</li>
                    <li>{format!("Negative: {}", self.props.gba.borrow().cpu.cpsr.flags.negative)}</li>
                    <li>{format!("Signed Overflow: {}", self.props.gba.borrow().cpu.cpsr.flags.signed_overflow)}</li>
                    <li>{format!("Zero: {}", self.props.gba.borrow().cpu.cpsr.flags.zero)}</li>
                </ul>
            </div>
        }
    }
}