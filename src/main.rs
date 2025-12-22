use log::{Level, info};
use std::collections::HashMap;
use web_sys;
use yew::prelude::*;

mod about;
// mod blog;
// mod cv;
mod footer;
mod header;
// mod home;
mod not_found;
// mod projects;
// mod utils;
mod window;
mod window_manager;

use crate::footer::Footer;
use crate::header::Header;
use crate::window::WindowProps;
use crate::window_manager::{WindowManager, WindowManagerProps};

pub fn get_current_path() -> AttrValue {
    AttrValue::from(
        web_sys::window()
            .expect("should have a window")
            .location()
            .pathname()
            .expect("should have a pathname"),
    )
}

#[function_component(App)]
pub fn app() -> Html {
    let current_page = use_state(get_current_path);
    let window_manager_props = WindowManagerProps {
        current_page: (*current_page).clone(),
        pages: HashMap::from([
            (
                AttrValue::from("/"),
                vec![WindowProps {
                    title: AttrValue::from("Home"),
                    content: yew::html::ChildrenRenderer::new(vec![html! {
                        <div>
                            <h2>{"Home"}</h2>
                            <p>{"This is your desktop home. Feel free to drag this window around!"}</p>
                            <ul>
                                <li>{"Rust version: 1.75+"}</li>
                                <li>{"Framework: Yew"}</li>
                            </ul>
                        </div>
                    }]),

                    x: 2.0,
                    y: 12.0,
                    width: 42.0,
                    height: 80.0,
                    buffer: 20.0,
                }],
            ),
            (
                AttrValue::from("/about"),
                about::get_about_windows(),
            ),
        ]),
    };

    html! {
        <div>
            <Header />
            <main>
                <WindowManager ..window_manager_props/>
            </main>
            <Footer />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();
    info!("Yew app starting...");
    yew::Renderer::<App>::new().render();
}
