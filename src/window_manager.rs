use crate::window::{WindowState, WindowWidget};
use leptos::prelude::*;
use std::collections::HashMap;

pub struct WindowManager<'a> {
    pages: HashMap<String, Vec<&'a WindowState>>,
}

impl<'a> WindowManager<'a> {
    pub fn new(pages: HashMap<String, Vec<&'a WindowState>>) -> Self {
        Self { pages }
    }

    fn render_page(&self, page: &str) {
        let page_windows = self.pages.get(page);
        let page_windows = match page_windows {
            Some(windows) => Some(windows.clone()),
            None => None,
        };
        todo!();
        // view! {
        //     <div>
        //         {move ||{
        //             if let Some(windows) = page_windows {
        //                 view! {
        //                     <For
        //                         each=move || windows
        //                         key=|window| window.id()
        //                         children=move |window| view! { <WindowWidget state=window /> }
        //                     />
        //                 }
        //             } else {
        //                 view! {<NotFoundPage />}
        //             }
        //         }}
        //     </div>
        // }
    }
}
