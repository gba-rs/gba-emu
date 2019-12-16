use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::console::ConsoleService;
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::InstructionSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Registers {
    props: RegistersProp,
    updated_reg_hex: String,
    updated_reg_dec: String,
    update_reg_num: u8
}

#[derive(Properties)]
pub struct RegistersProp {
    #[props(required)]
    pub gba: Rc<RefCell<GBA>>,
    #[props(required)]
    pub hex: bool
}

pub enum RegUpdateType {
    Hex,
    Dec
}

pub enum Msg {
    StartUpdate(String, RegUpdateType),
    UpdateReg(String, u8, RegUpdateType),
    FinishUpdate(RegUpdateType),
    Nope
}

impl Component for Registers {
    type Message = Msg;
    type Properties = RegistersProp;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Registers {
            props: props,
            updated_reg_dec: "".to_string(),
            updated_reg_hex: "".to_string(),
            update_reg_num: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartUpdate(init_string, update_type) => {
                match update_type {
                    RegUpdateType::Dec => {
                        self.updated_reg_dec = init_string;
                    },
                    RegUpdateType::Hex => {
                        self.updated_reg_hex = init_string;
                    }
                }
            },
            Msg::UpdateReg(update_string, reg_num, update_type) => {
                self.update_reg_num = reg_num;
                match update_type {
                    RegUpdateType::Hex => {
                        self.updated_reg_hex = update_string;
                    },
                    RegUpdateType::Dec => {
                        self.updated_reg_dec = update_string;
                    }
                }
            },
            Msg::FinishUpdate(update_type) => {
                match update_type {
                    RegUpdateType::Hex => {
                        self.updated_reg_hex.retain(|c| !c.is_whitespace());
                        let result = u32::from_str_radix(&self.updated_reg_hex, 16);
                        match result {
                            Ok(val) => {
                                self.props.gba.borrow_mut().cpu.set_register(self.update_reg_num, val)
                            },
                            Err(_) => {
                                ConsoleService::new().log(&format!("Error updating r{}: {}", self.update_reg_num, self.updated_reg_hex));
                            }
                        }                    
                    },
                    RegUpdateType::Dec => {
                        self.updated_reg_dec.retain(|c| !c.is_whitespace());
                        let result = u32::from_str_radix(&self.updated_reg_dec, 10);
                        match result {
                            Ok(val) => {
                                self.props.gba.borrow_mut().cpu.set_register(self.update_reg_num, val)
                            },
                            Err(_) => {
                                ConsoleService::new().log(&format!("Error updating r{}: {}", self.update_reg_num, self.updated_reg_dec));
                            }
                        }                    
                    }
                }
            },
            Msg::Nope => {}
        }
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
                <table class="table register-table">
                    <thead>
                        <tr>
                            <th scope="col">{"Reg"}</th>
                            <th scope="col">{"Val Hex"}</th>
                            <th scope="col">{"Val Dec"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for (0..if self.props.gba.borrow().cpu.current_instruction_set == InstructionSet::Arm { 16 } else { 10 }).map(|val|{
                            let reg_val = self.props.gba.borrow().cpu.get_register(val);
                            let reg_num = val;
                            html! {
                                <tr>
                                    <td class="text-left">{format!("r{}", val)}</td>
                                    <td class="text-right">
                                        <input class="hex-edit hex-edit-word" type="text" value={format!("{:08X}", reg_val)} 
                                        onclick=|_|{ Msg::StartUpdate(format!("{:08X}", reg_val), RegUpdateType::Hex) }
                                        oninput=|e|{ Msg::UpdateReg(e.value, reg_num, RegUpdateType::Hex) }
                                        onkeypress=|e|{ if e.key() == "Enter" { Msg::FinishUpdate(RegUpdateType::Hex) } else { Msg::Nope } }
                                        />
                                    </td>
                                    <td class="text-right">
                                        <input class="hex-edit hex-edit-word" type="text" value={format!("{}", reg_val)}
                                        onclick=|_|{ Msg::StartUpdate(format!("{}", reg_val), RegUpdateType::Dec) }
                                        oninput=|e|{ Msg::UpdateReg(e.value, reg_num, RegUpdateType::Dec) }
                                        onkeypress=|e|{ if e.key() == "Enter" { Msg::FinishUpdate(RegUpdateType::Dec) } else { Msg::Nope } }
                                        />
                                    </td>
                                </tr>
                            }                            
                        })}
                    </tbody>
                </table>
            </div>
        }
    }
}