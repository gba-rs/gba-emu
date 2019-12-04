use yew::prelude::*;
use gba_emulator::gba::GBA;
use std::fs::File;
use std::io::prelude::*;
use std::env;


pub struct App {
    // gba: GBA
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        // let rom_file = File::open("roms/fib-3.rom");
        // let mut rom = Vec::new();
        // let _ = rom_file.unwrap().read_to_end(&mut rom);
    
        // let bios_file = File::open(&"roms/GBA.BIOS");
        // let mut bios = Vec::new();
        // let _ = bios_file.unwrap().read_to_end(&mut bios);
    
        App {
            // gba: GBA::new(0x08000000, bios, rom)
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <p>{ "Fuck Everything" }</p>
        }
    }
}
