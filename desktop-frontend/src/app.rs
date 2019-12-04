use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::services::console::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::InstructionSet;


pub struct App {
    // gba: GBA
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
    rom: Vec<u8>,
    bios: Vec<u8>,
    gba: GBA,
    link: ComponentLink<App>,
    hex: bool
}

pub enum Msg {
    LoadedRom(FileData),
    LoadedBios(FileData),
    Init,
    Step(u8),
    ToggleHex,
    Files(Vec<File>, bool)
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
            gba: GBA::new_default(),
            tasks: vec![],
            hex: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadedRom(file) => {
                self.rom = file.content;
            },
            Msg::LoadedBios(file) => {
                self.bios = file.content;
            },
            Msg::Init => {
                self.gba = GBA::new(0x08000000, &self.bios, &self.rom);
                ConsoleService::new().log("Created new Emulator");
            },
            Msg::Step(step_count) => {
                for _ in 0..step_count {
                    self.gba.step();
                    ConsoleService::new().log("Step");
                }
            },
            Msg::ToggleHex => {
                self.hex = !self.hex;
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
                <div class="navbar">
                    <a>{"GBA Emulator"}</a>
                    <a>{"Debugger"}</a>
                </div>
                <div class="main">
                    <div class="card row">
                        <label for="bios-filechooser">{"Bios: "}</label>
                        <input type="file" id="bios-filechooser" onchange=|value| {
                                let mut result = Vec::new();
                                if let ChangeData::Files(files) = value {
                                    result.extend(files);
                                }
                                Msg::Files(result, false)
                            }/>

                        <label for="rom-filechooser">{"Rom: "}</label>
                        <input type="file" id="rom-filechooser" onchange=|value| {
                                let mut result = Vec::new();
                                if let ChangeData::Files(files) = value {
                                    result.extend(files);
                                }
                                Msg::Files(result, true)
                            }/>
                        <button class="button-flat" onclick=|_|{Msg::Init}>{"Init Emulator"}</button>
                        <button class="button-flat" onclick=|_|{Msg::Step(1)}>{"Step"}</button>
                        <button class="button-flat" onclick=|_|{Msg::ToggleHex}>{"Toggle Hex"}</button>
                    </div>
                    <div class="card row">
                        <h3>{"Status"}</h3>
                        <p>{format!("Instruction Set: {:?}", self.gba.cpu.current_instruction_set)}</p>
                        <p>{format!("Operating Mode: {:?}", self.gba.cpu.operating_mode)}</p>
                        <p>{&self.gba.cpu.last_instruction}</p>
                    </div>
                    <div class="card row">
                        <h3>{"Registers"}</h3>
                        <ul>
                            {for (0..if self.gba.cpu.current_instruction_set == InstructionSet::Arm { 16 } else { 10 }).map(|val|{
                                if self.hex {
                                    html! {
                                        <li>{format!("R{} = 0x{:X}", val, self.gba.cpu.get_register(val))}</li>
                                    }
                                } else {
                                    html! {
                                        <li>{format!("R{} = {}", val, self.gba.cpu.get_register(val))}</li>
                                    }
                                }
                                
                            })}
                        </ul>
                    </div>
                    <div class="card row">
                        <h3>{"Current Program Status Register"}</h3>
                        <ul>
                            <li>{format!("Carry: {}", self.gba.cpu.cpsr.flags.carry)}</li>
                            <li>{format!("Negative: {}", self.gba.cpu.cpsr.flags.negative)}</li>
                            <li>{format!("Signed Overflow: {}", self.gba.cpu.cpsr.flags.signed_overflow)}</li>
                            <li>{format!("Zero: {}", self.gba.cpu.cpsr.flags.zero)}</li>
                        </ul>
                    </div>
                </div>
            </>
        }
    }
}