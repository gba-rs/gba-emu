use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::{InstructionSet, OperatingMode};
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

pub enum Msg {
    UpdateInstructionSet(InstructionSet),
    UpdateOperatingMode(OperatingMode)
}

impl Component for Status {
    type Message = Msg;
    type Properties = StatusProp;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Status {
            props: props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateInstructionSet(instr_set) => {
                self.props.gba.borrow_mut().cpu.current_instruction_set = instr_set;
            },
            Msg::UpdateOperatingMode(op_mode) => {
                self.props.gba.borrow_mut().cpu.operating_mode = op_mode;
            }
        }
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
                <div class="dropdown m-2">
                    <button class="btn btn-outline-primary dropdown-toggle" type="button" data-toggle="dropdown">
                        {&format!("{:?}", self.props.gba.borrow().cpu.current_instruction_set)}
                    </button>
                    <div class="dropdown-menu">
                        <button class="dropdown-item" type="button" onclick=|_|{Msg::UpdateInstructionSet(InstructionSet::Arm)}>{"Arm"}</button>
                        <button class="dropdown-item" type="button" onclick=|_|{Msg::UpdateInstructionSet(InstructionSet::Thumb)}>{"Thumb"}</button>
                    </div>
                </div>
                <div class="dropdown m-2">
                    <button class="btn btn-outline-primary dropdown-toggle" type="button" data-toggle="dropdown">
                        {&format!("{:?}", self.props.gba.borrow().cpu.operating_mode)}
                    </button>
                    <div class="dropdown-menu">
                        <button class="dropdown-item" type="button" onclick=|_|{Msg::UpdateOperatingMode(OperatingMode::System)}>{"System"}</button>
                        <button class="dropdown-item" type="button" onclick=|_|{Msg::UpdateOperatingMode(OperatingMode::User)}>{"User"}</button>
                        <button class="dropdown-item" type="button" onclick=|_|{Msg::UpdateOperatingMode(OperatingMode::FastInterrupt)}>{"Fast Interrupt"}</button>
                        <button class="dropdown-item" type="button" onclick=|_|{Msg::UpdateOperatingMode(OperatingMode::Supervisor)}>{"Supervisor"}</button>
                        <button class="dropdown-item" type="button" onclick=|_|{Msg::UpdateOperatingMode(OperatingMode::Abort)}>{"Abort"}</button>
                        <button class="dropdown-item" type="button" onclick=|_|{Msg::UpdateOperatingMode(OperatingMode::Interrupt)}>{"Interrupt"}</button>
                        <button class="dropdown-item" type="button" onclick=|_|{Msg::UpdateOperatingMode(OperatingMode::Undefined)}>{"Undefined"}</button>
                    </div>
                </div>
            </div>
        }
    }
}