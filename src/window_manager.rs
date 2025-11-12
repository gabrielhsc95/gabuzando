use std::collections::HashMap;
use yew::prelude::*;

use crate::not_found::PageNotFound;
use crate::window::{WindowProps, WindowWidget};

#[derive(Properties, PartialEq)]
pub struct WindowManagerProps {
    pub current_page: AttrValue,
    pub pages: HashMap<AttrValue, Vec<WindowProps>>, // I might be able to use Children here
}
#[function_component(WindowManager)]
pub fn window_manager(props: &WindowManagerProps) -> Html {
    let current_page = &props.pages.get(&props.current_page);
    html! {
        match current_page {
            Some(windows) => {
                html! {
                    { for windows.iter().map(|w_props| html! { <WindowWidget ..w_props.clone() /> }) }
                }
            },
            None => html! {<PageNotFound />}
        }
    }
}
