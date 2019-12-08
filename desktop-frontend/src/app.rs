use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::services::console::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::InstructionSet;
use std::rc::Rc;
use std::cell::RefCell;

use crate::components::{
    registers::Registers, 
    navbar::NavBar, 
    cpsr::Cpsr,
    status::Status
};


pub struct App {
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
    rom: Vec<u8>,
    bios: Vec<u8>,
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
            gba: Rc::new(RefCell::new(GBA::default())),
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
                self.gba = Rc::new(RefCell::new(GBA::new(0x08000000, &self.bios, &self.rom)));
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
                    <Status gba={self.gba.clone()}/>
                    <Registers hex={self.hex} gba={self.gba.clone()}/>
                    <Cpsr gba={self.gba.clone()}/>
                </div>
            </>
        }
    }
}