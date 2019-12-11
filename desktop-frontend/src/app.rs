use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::services::console::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::{cpu::InstructionSet, cpu::ARM_PC};
use gba_emulator::formats::common::Instruction;
use std::rc::Rc;
use std::cell::RefCell;

use crate::components::{
    registers::Registers, 
    navbar::NavBar, 
    cpsr::Cpsr,
    status::Status
};

pub const start_pc: u32 = 0x08000000;

struct DisassemblyElement {
    address: u32,
    instruction_hex: u32,
    instruction_asm: String,
    selected: bool
}


pub struct App {
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
    rom: Vec<u8>,
    bios: Vec<u8>,
    rom_name: String,
    bios_name: String,
    disassembly: Vec<DisassemblyElement>,
    gba: Rc<RefCell<GBA>>,
    link: ComponentLink<App>,
    hex: bool
}

pub enum Msg {
    LoadedRom(FileData),
    LoadedBios(FileData),
    Init,
    Step(u8),
    ToggleHex,
    Files(Vec<File>, bool),
    Disassemble(InstructionSet)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::new().log("Created Application");
        App {
            reader: ReaderService::new(),
            link,
            bios: vec![],
            rom: vec![],
            rom_name: "Choose File".to_string(),
            bios_name: "Choose File".to_string(),
            disassembly: vec![],
            gba: Rc::new(RefCell::new(GBA::default())),
            tasks: vec![],
            hex: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadedRom(file) => {
                self.rom = file.content;
                self.rom_name = file.name;
            },
            Msg::LoadedBios(file) => {
                self.bios = file.content;
                self.bios_name = file.name;
            },
            Msg::Init => {
                self.gba = Rc::new(RefCell::new(GBA::new(start_pc, &self.bios, &self.rom)));
                ConsoleService::new().log("Created new Emulator");
            },
            Msg::Step(step_count) => {
                for _ in 0..step_count {
                    self.gba.as_ref().borrow_mut().step();
                    ConsoleService::new().log("Step");
                }
            },
            Msg::ToggleHex => {
                self.hex = !self.hex;
            },
            Msg::Disassemble(instr_set) => {
                self.disassembly.clear();
                ConsoleService::new().log(&format!("Rom size: {}", self.rom.len()));
                match instr_set {
                    InstructionSet::Arm => {
                        for i in (0..if self.rom.len() > 10000 {10000} else {self.rom.len()}).step_by(4) {
                            let instruction: u32 = self.rom[i] as u32 | 
                            ((self.rom[i as usize + 1] as u32) << 8) | 
                            ((self.rom[i as usize + 2] as u32) << 16) | 
                            ((self.rom[i as usize + 3] as u32) << 24);

                            let decode_result = self.gba.borrow().cpu.decode(instruction);
                            match decode_result {
                                Ok(decoded_instruction) => {
                                    self.disassembly.push(DisassemblyElement{
                                        address: (i as u32) + start_pc,
                                        instruction_hex: instruction,
                                        instruction_asm: decoded_instruction.asm(),
                                        selected: ((i as u32) + start_pc) == self.gba.borrow().cpu.get_register(ARM_PC)
                                    });
                                },
                                Err(e) => {
                                    self.disassembly.push(DisassemblyElement {
                                        address: (i as u32) + start_pc,
                                        instruction_hex: instruction,
                                        instruction_asm: "???".to_string(),
                                        selected: ((i as u32) + start_pc) == self.gba.borrow().cpu.get_register(ARM_PC)
                                    });
                                }
                            }

                            
                        }
                    },
                    InstructionSet::Thumb => {

                    }
                }
            },
            Msg::Files(files, rom) => {
                for file in files.into_iter() {
                    let task = {
                        if rom {
                            let callback = self.link.send_back(Msg::LoadedRom);
                            self.reader.read_file(file, callback)
                        } else {
                            let callback = self.link.send_back(Msg::LoadedBios);
                            self.reader.read_file(file, callback)
                        }
                    };
                    self.tasks.push(task);
                }
            }
        }

        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <NavBar/>
                <div class="container-fluid">
                    <div class="row">
                        <div class="col-3">
                            <h4>{"Control"}</h4>
                            <div>                               
                                <div class="input-group mb-3">
                                    <div class="input-group-prepend">
                                        <span class="input-group-text" id="inputGroupFileAddon01">{"Bios"}</span>
                                    </div>
                                    <div class="custom-file">
                                        <input type="file" class="custom-file-input" id="inputGroupFile01" aria-describedby="inputGroupFileAddon01" onchange=|value| {
                                            let mut result = Vec::new();
                                            if let ChangeData::Files(files) = value {
                                                result.extend(files);
                                            }
                                            Msg::Files(result, false)
                                        }/>
                                        <label class="custom-file-label" for="inputGroupFile01">{format!("{}", self.bios_name)}</label>
                                    </div>
                                </div>
                            </div>

                            <div>                               
                                <div class="input-group mb-3">
                                    <div class="input-group-prepend">
                                        <span class="input-group-text" id="inputGroupFileAddon02">{"Rom"}</span>
                                    </div>
                                    <div class="custom-file">
                                        <input type="file" class="custom-file-input" id="inputGroupFile02" aria-describedby="inputGroupFileAddon02" onchange=|value| {
                                            let mut result = Vec::new();
                                            if let ChangeData::Files(files) = value {
                                                result.extend(files);
                                            }
                                            Msg::Files(result, true)
                                        }/>
                                        <label class="custom-file-label" for="inputGroupFile02">{format!("{}", self.rom_name)}</label>
                                    </div>
                                </div>
                            </div>
                        
                            <div class="control">
                                <button class="btn btn-outline-primary" onclick=|_|{Msg::Init}>{"Init Emulator"}</button>
                                <button class="btn btn-outline-primary" onclick=|_|{Msg::Step(1)}>{"Step"}</button>
                                <button class="btn btn-outline-primary" onclick=|_|{Msg::ToggleHex}>{"Toggle Hex"}</button>
                                <button class="btn btn-outline-primary" onclick=|_|{Msg::Disassemble(InstructionSet::Arm)}>{"Disassemble"}</button>
                            </div>
                            <Status gba={self.gba.clone()}/>
                            <Cpsr gba={self.gba.clone()}/>
                        </div>
                        
                        <div class="col-2">
                            <Registers hex={self.hex} gba={self.gba.clone()}/>
                        </div>
                        
                    
                        <div class="col-7">
                            <div class="code-block">
                                {for (0..self.disassembly.len()).map(|val|{
                                    html! {
                                        <div class={if self.disassembly[val].address == self.gba.borrow().cpu.get_register(ARM_PC) {"disassembly-selected"} else {""}}>
                                            <span class="disassembly-address">{format!("{:08X}", self.disassembly[val].address)}</span>
                                            <span class="disassembly-hex">{format!("{:08X}", self.disassembly[val].instruction_hex)}</span>
                                            <span class="disassembly-asm">{format!("{}", self.disassembly[val].instruction_asm)}</span>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}