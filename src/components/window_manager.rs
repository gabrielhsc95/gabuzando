use yew::prelude::*;

use crate::pages::not_found::PageNotFound;
use crate::components::window::{WindowProps, WindowWidget};

#[derive(Properties, PartialEq)]
pub struct WindowManagerProps {
    pub windows: Option<Vec<WindowProps>>,
}

#[function_component(WindowManager)]
pub fn window_manager(props: &WindowManagerProps) -> Html {
    html! {
        match &props.windows {
            Some(windows) => {
                html! {
                    { for windows.iter().map(|w_props| html! { <WindowWidget ..w_props.clone() /> }) }
                }
            },
            None => html! {<PageNotFound />}
        }
    }
}
