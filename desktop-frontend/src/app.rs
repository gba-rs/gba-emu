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
    status::Status,
    memory_viewer::MemoryViewer
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
    hex: bool,
    follow_pc: bool,
    initialized: bool,
    disassembled: bool,
    dis_min: u32,
    dis_max: u32,
    mem_min: u32,
    mem_max: u32,
    dis_min_str: String,
    dis_max_str: String,
    mem_min_str: String,
    mem_max_str: String
}

pub enum RangeUpdate{
    MemoryViewerMin,
    MemoryViewerMax,
    DisassemblyMin,
    DisassemblyMax
}

pub enum Msg {
    LoadedRom(FileData),
    LoadedBios(FileData),
    Init,
    Step(u8),
    ToggleHex,
    Files(Vec<File>, bool),
    Disassemble(InstructionSet),
    ToggleFollow,
    UpdateRange(RangeUpdate),
    UpdateInputString(String, RangeUpdate)
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
            hex: false,
            follow_pc: true,
            initialized: false,
            disassembled: false,
            dis_min: 0,
            dis_max: 100,
            mem_min: 0,
            mem_max: 100,
            dis_min_str: "".to_string(),
            dis_max_str: "".to_string(),
            mem_min_str: "".to_string(),
            mem_max_str: "".to_string()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadedRom(file) => {
                self.rom = file.content;
                self.rom_name = file.name;
                self.disassembled = false;
                self.initialized = false;
                true
            },
            Msg::LoadedBios(file) => {
                self.bios = file.content;
                self.bios_name = file.name;
                self.initialized = false;
                true
            },
            Msg::Init => {
                self.gba = Rc::new(RefCell::new(GBA::new(start_pc, &self.bios, &self.rom)));
                self.initialized = true;
                ConsoleService::new().log("Created new Emulator");
                true
            },
            Msg::Step(step_count) => {
                for _ in 0..step_count {
                    self.gba.as_ref().borrow_mut().step();
                    ConsoleService::new().log("Step");
                }
                true
            },
            Msg::ToggleHex => {
                self.hex = !self.hex;
                true
            },
            Msg::ToggleFollow => {
                self.follow_pc = !self.follow_pc;
                true
            },
            Msg::UpdateRange(range_to_update) => {
                match range_to_update {
                    RangeUpdate::MemoryViewerMin | RangeUpdate::MemoryViewerMax => {
                        let result = u32::from_str_radix(&self.mem_max_str, 16);//self.mem_max_str.parse::<u32>();
                        match result {
                            Ok(val) => {
                                self.mem_max = val;
                            },
                            Err(e) => {}
                        }

                        let result = u32::from_str_radix(&self.mem_min_str, 16);
                        match result {
                            Ok(val) => {
                                self.mem_min = val;
                            },
                            Err(e) => {}
                        }
                    },
                    RangeUpdate::DisassemblyMin | RangeUpdate::DisassemblyMax => {
                        let result = u32::from_str_radix(&self.dis_max_str, 16);
                        match result {
                            Ok(val) => {
                                self.dis_max = val;
                            },
                            Err(e) => {}
                        }

                        let result = u32::from_str_radix(&self.dis_min_str, 16);
                        match result {
                            Ok(val) => {
                                self.dis_min = val;
                            },
                            Err(e) => {}
                        }
                    }
                }
                true
            },
            Msg::UpdateInputString(val, range_to_update) => {
                match range_to_update {
                    RangeUpdate::MemoryViewerMin => {
                        self.mem_min_str = val;
                    },
                    RangeUpdate::MemoryViewerMax => {
                        self.mem_max_str = val;
                    },
                    RangeUpdate::DisassemblyMin => {
                        self.dis_min_str = val;
                    },
                    RangeUpdate::DisassemblyMax => {
                        self.dis_max_str = val;
                    }
                }
                false
            }
            Msg::Disassemble(instr_set) => {
                self.disassembly.clear();
                ConsoleService::new().log(&format!("Rom size: {}", self.rom.len()));
                match instr_set {
                    InstructionSet::Arm => {
                        for i in (0..self.rom.len()).step_by(4) {
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

                        self.disassembled = true;
                    },
                    InstructionSet::Thumb => {

                    }
                }
                true
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
                false
            }
        }
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <NavBar/>
                <div class="container-fluid">
                    <div class="row">
                        {self.view_control()}
                    </div>
                    <div class="row">

                         <div class="col-xs-12 col-lg-6 col-xl-3">
                            <Status gba={self.gba.clone()}/>
                            <Cpsr gba={self.gba.clone()}/>
                        </div>
                        
                        <div class="col-xs-12 col-lg-6 col-xl-3">
                            <Registers hex={self.hex} gba={self.gba.clone()}/>
                        </div>
                    
                        <div class="col-xs-12 col-xl-6">
                            <div class="row">
                                <div class="col-3">
                                    {self.view_range_dis()}
                                </div>
                                <div class="col-9">
                                    {self.view_disassembly()}
                                </div>
                            </div>
                            <div class="row">
                                <div class="col-3">
                                    {self.view_range_mem()}
                                </div>
                                <div class="col-9">
                                    <MemoryViewer gba={self.gba.clone()} min={self.mem_min} max={self.mem_max} initialized={self.initialized}/>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

impl App {
    pub fn view_disassembly(&self) -> Html<Self> {
        if self.disassembled {
            let current_pc = self.gba.borrow().cpu.get_register(ARM_PC);
            let mut disassembly_start: i64;
            
            let mut disassembly_end: i64;

            if self.follow_pc {
                disassembly_start = ((current_pc as i64 - start_pc as i64) - 100) / 4;
                disassembly_end = ((current_pc as i64 - start_pc as i64) + 100) / 4;
            } else {
                disassembly_start = (self.dis_min as i64 - start_pc as i64) / 4;
                disassembly_end = (self.dis_max as i64 - start_pc as i64) / 4;
            }

            if disassembly_start < 0 {
                disassembly_start = 0;
            }

            if disassembly_end < 0 {
                disassembly_end = 0;
            }

            if disassembly_end > self.disassembly.len() as i64{
                disassembly_end = self.disassembly.len() as i64;
            }

            html! {
                <div class="code-block">
                    {for (disassembly_start..disassembly_end).map(|val|{
                        let element = &self.disassembly[val as usize];

                        html! {
                            <div class={if self.disassembly[val as usize].address == current_pc {"disassembly-selected"} else {""}}>
                                <span class="disassembly-address">{format!("{:08X}", element.address)}</span>
                                <span class="disassembly-hex">{format!("{:08X}", element.instruction_hex)}</span>
                                <span class="disassembly-asm">{format!("{}", element.instruction_asm)}</span>
                            </div>
                        }
                    })}
                </div>
            }
        } else {
            html! {
                <div class="code-block">{"Run Disassembly"}</div>
            }
        }
    }

    pub fn view_range_dis(&self) -> Html<Self> {
        html! {
            <>
                <h5>{"Disassembly"}</h5>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="lower-addon-dis">{"Lower"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="0" oninput=|e| {Msg::UpdateInputString(e.value, RangeUpdate::DisassemblyMin)}/>
                </div>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="upper-addon-dis">{"Upper"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="100" oninput=|e| {Msg::UpdateInputString(e.value, RangeUpdate::DisassemblyMax)}/>
                </div>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="follow-addon">{"Follow PC"}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={self.follow_pc} onclick=|_|{Msg::ToggleFollow}/>
                        </div>                                
                    </div>
                </div>
                <button class="btn btn-outline-primary" onclick=|_|{Msg::UpdateRange(RangeUpdate::DisassemblyMax)}>{"Search"}</button>
            </>
        }
    }

    pub fn view_range_mem(&self) -> Html<Self> {
        html!{
            <>
                <h5>{"Memory"}</h5>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="lower-addon-mem">{"Lower"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="0" oninput=|e| {Msg::UpdateInputString(e.value, RangeUpdate::MemoryViewerMin)}/>
                </div>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="upper-addon-mem">{"Upper"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="100" oninput=|e| {Msg::UpdateInputString(e.value, RangeUpdate::MemoryViewerMax)}/>
                </div>
                <button class="btn btn-outline-primary" onclick=|_|{Msg::UpdateRange(RangeUpdate::MemoryViewerMax)}>{"Search"}</button>
            </>
        }
    }

    pub fn view_control(&self) -> Html<Self> {
        html! {
            <>
                // <h4>{"Control"}</h4>
                <div class="col-xs-12 col-md-6 col-xl-3">                               
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

                <div class="col-xs-12 col-md-6 col-xl-3">                               
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
            
                <div class="col-xs-12 col-xl-6 sticky-top">
                    <div class="btn-group" role="group">
                        <button class="btn btn-outline-primary" onclick=|_|{Msg::Init}>{"Init Emulator"}</button>
                        <button class="btn btn-outline-primary" onclick=|_|{Msg::Step(1)}>{"Step"}</button>
                        <button class="btn btn-outline-primary" onclick=|_|{Msg::Disassemble(InstructionSet::Arm)}>{"Disassemble"}</button>
                    </div>
                </div>
                
                // <Status gba={self.gba.clone()}/>
                // <Cpsr gba={self.gba.clone()}/>
            </>
        }
    }
}