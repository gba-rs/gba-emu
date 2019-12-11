use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::console::ConsoleService;
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MemoryViewer {
    props: MemoryViewerProp
}

#[derive(Properties)]
pub struct MemoryViewerProp {
    #[props(required)]
    pub gba: Rc<RefCell<GBA>>,
    #[props(required)]
    pub min: u32,
    #[props(required)]
    pub max: u32,
    #[props(required)]
    pub initialized: bool
}

pub enum Msg {}

impl Component for MemoryViewer {
    type Message = Msg;
    type Properties = MemoryViewerProp;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        MemoryViewer {
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

impl Renderable<MemoryViewer> for MemoryViewer {
    fn view(&self) -> Html<Self> {
        if self.props.initialized {
            ConsoleService::new().log("Attempting to get the bytes");
            let bytes = self.props.gba.borrow().mem_map.read_block(self.props.min, self.props.max - self.props.min);
            ConsoleService::new().log(&format!("Got the bytes: {}", bytes.len()));
            html! {
                <div class="code-block">
                    {for (0..bytes.len()).step_by(16).map(|val|{
                        html!{
                            <div>
                                <span class="disassembly-address">{format!("{:08X}", (self.props.min + val as u32))}</span>
                                {for (0..16).map(|offset|{
                                    let index = val + offset;
                                    if index < bytes.len() {
                                        html! {
                                            <span>{format!(" {:02X}", bytes[val + offset])}</span>
                                        }
                                    } else {
                                        html! {
                                            <span>{format!(" --",)}</span>
                                        }
                                    }
                                })}
                            </div>
                        }
                    })}
                </div>
            }
        } else {
            html! {
                <div class="code-block">{"Initialize the emulator"}</div>
            }
        }
    }
}