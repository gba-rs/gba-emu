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

pub enum UpdateFlagType{
    Carry,
    Negative,
    SignedOverflow,
    Zero
}

pub enum Msg {
    UpdateFlag(UpdateFlagType)
}

impl Component for Cpsr {
    type Message = Msg;
    type Properties = CpsrProp;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Cpsr {
            props: props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let flags = self.props.gba.borrow().cpu.cpsr.flags.clone();
        match msg {
            Msg::UpdateFlag(flag_to_update) => {
                match flag_to_update {
                    UpdateFlagType::Carry => {
                        self.props.gba.borrow_mut().cpu.cpsr.flags.carry = !flags.carry;
                    },
                    UpdateFlagType::Negative => {
                        self.props.gba.borrow_mut().cpu.cpsr.flags.negative = !flags.negative;
                    },
                    UpdateFlagType::SignedOverflow => {
                        self.props.gba.borrow_mut().cpu.cpsr.flags.signed_overflow = !flags.signed_overflow;
                    },
                    UpdateFlagType::Zero => {
                        self.props.gba.borrow_mut().cpu.cpsr.flags.zero = !flags.zero;
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl Renderable<Cpsr> for Cpsr {
    fn view(&self) -> Html<Self> {
        let flags = self.props.gba.borrow().cpu.cpsr.flags.clone();

        html! {
            <div class="col-12">
                <h4>{"Current Program Status Register"}</h4>
                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("Carry - {:?}", flags.carry)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={flags.carry} onclick=|_|{Msg::UpdateFlag(UpdateFlagType::Carry)}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("Negative - {:?}", flags.negative)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={flags.negative} onclick=|_|{Msg::UpdateFlag(UpdateFlagType::Negative)}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("Signed Overflow - {:?}", flags.signed_overflow)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={flags.signed_overflow} onclick=|_|{Msg::UpdateFlag(UpdateFlagType::SignedOverflow)}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("Zero - {:?}", flags.zero)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={flags.zero} onclick=|_|{Msg::UpdateFlag(UpdateFlagType::Zero)}/>
                        </div>
                    </div>
                </div>
            </div>
            
        }
    }
}