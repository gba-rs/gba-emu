use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::InstructionSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Registers {
    props: RegistersProp
}

#[derive(Properties)]
pub struct RegistersProp {
    #[props(required)]
    pub gba: Rc<RefCell<GBA>>,
    #[props(required)]
    pub hex: bool
}

pub enum Msg {}

impl Component for Registers {
    type Message = Msg;
    type Properties = RegistersProp;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Registers {
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

impl Renderable<Registers> for Registers {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="card row">
                <h3>{"Registers"}</h3>
                <ul>
                    {for (0..if self.props.gba.borrow().cpu.current_instruction_set == InstructionSet::Arm { 16 } else { 10 }).map(|val|{
                        if self.props.hex {
                            html! {
                                <li>{format!("R{} = 0x{:X}", val, self.props.gba.borrow().cpu.get_register(val))}</li>
                            }
                        } else {
                            html! {
                                <li>{format!("R{} = {}", val, self.props.gba.borrow().cpu.get_register(val))}</li>
                            }
                        }
                        
                    })}
                </ul>
            </div>
        }
    }
}