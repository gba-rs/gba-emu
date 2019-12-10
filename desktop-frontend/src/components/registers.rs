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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Registers {
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

impl Renderable<Registers> for Registers {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h4>{"Registers"}</h4>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">{"Reg"}</th>
                            <th scope="col">{"Val"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for (0..if self.props.gba.borrow().cpu.current_instruction_set == InstructionSet::Arm { 16 } else { 10 }).map(|val|{
                            if self.props.hex {
                                html! {
                                    <tr>
                                        <td>{format!("{}", val)}</td>
                                        <td>{format!("0x{:X}", self.props.gba.borrow().cpu.get_register(val))}</td>
                                    </tr>
                                }
                            } else {
                                html! {
                                    <tr>
                                        <td>{format!("{}", val)}</td>
                                        <td>{format!("{}", self.props.gba.borrow().cpu.get_register(val))}</td>
                                    </tr>
                                }
                            }
                            
                        })}
                    </tbody>
                </table>
            </div>
        }
    }
}