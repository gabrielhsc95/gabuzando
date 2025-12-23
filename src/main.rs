use log::{Level, info};
use std::collections::HashMap;
use web_sys;
use yew::prelude::*;

mod components;
mod pages;
mod types;
mod hooks;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::language_context::LanguageProvider;
use crate::components::window_manager::{WindowManager, WindowManagerProps};
use crate::pages::{about, blog, cv, home, projects};

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
                home::get_home_windows(),
            ),
            (
                AttrValue::from("/about"),
                about::get_about_windows(),
            ),
            (
                AttrValue::from("/cv"),
                cv::get_cv_windows(),
            ),
            (
                AttrValue::from("/blog"),
                blog::get_blog_windows(),
            ),
            (
                AttrValue::from("/projects"),
                projects::get_projects_windows(),
            ),
        ]),
    };

    html! {
        <LanguageProvider>
            <div>
                <Header />
                <main>
                    <WindowManager ..window_manager_props/>
                </main>
                <Footer />
            </div>
        </LanguageProvider>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();
    info!("Gabuzando starting...");
    yew::Renderer::<App>::new().render();
}
