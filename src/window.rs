use leptos::ev::MouseEvent;
use leptos::html::Div;
use leptos::prelude::*;
pub struct WindowState<Content: IntoView + 'static> {
    title: String,
    content: Content,
    x: RwSignal<i32>,
    y: RwSignal<i32>,
    width: i32,
    height: i32,
    is_minimized: RwSignal<bool>,
    is_maximized: RwSignal<bool>,
    is_opened: RwSignal<bool>,
    drag: RwSignal<Option<(i32, i32)>>,
}

impl<Content: IntoView + 'static> WindowState<Content> {
    pub fn new(title: String, content: Content, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            title,
            content,
            x: RwSignal::new(x),
            y: RwSignal::new(y),
            width,
            height,
            is_minimized: RwSignal::new(false),
            is_maximized: RwSignal::new(false),
            is_opened: RwSignal::new(true),
            drag: RwSignal::new(None),
        }
    }
}

#[component]
pub fn WindowWidget<Content: IntoView + 'static>(state: WindowState<Content>) -> impl IntoView {
    let div_ref: NodeRef<Div> = NodeRef::new();
    let body_style = RwSignal::new(String::from(""));

    let is_maximized_toggle = move |_| {
        state
            .is_maximized
            .update(|maximize: &mut bool| *maximize = !*maximize);
    };

    let is_minimized_toggle = move |_| {
        state
            .is_minimized
            .update(|minimize: &mut bool| *minimize = !*minimize);
    };

    let is_opened_toggle = move |_| {
        state.is_opened.update(|open: &mut bool| *open = !*open);
    };

    let start_drag = move |mouse_event: MouseEvent| {
        state
            .drag
            .update(|drag| *drag = Some((mouse_event.client_x(), mouse_event.client_y())));
        body_style.set("user-select: none;".to_string());
    };

    let drag_window = move |mouse_event: MouseEvent| {
        if state.drag.read().is_some() {
            let (initial_x, initial_y) = state.drag.read().unwrap();
            let mouse_x = mouse_event.client_x();
            let mouse_y = mouse_event.client_y();
            let delta_x = mouse_x - initial_x;
            let delta_y = mouse_y - initial_y;
            state
                .drag
                .update(|drag| *drag = Some((mouse_event.client_x(), mouse_event.client_y())));
            state.x.update(|x| *x += delta_x);
            state.y.update(|y| *y += delta_y);
        }
    };

    let stop_drag = move |_| {
        state.drag.update(|drag| *drag = None);
        body_style.set("".to_string());
    };

    view! {
        <div
            class="window"
            node_ref=div_ref
            style:width=move || {
                if *state.is_maximized.read() {
                    "98%".to_string()
                } else {
                    format!("{}px", state.width)
                }
            }
            style:height=move || {
                if *state.is_minimized.read() {
                    "37px".to_string()
                }
                else if *state.is_maximized.read(){
                    "87%".to_string()
                }
                else {
                    format!("{}px", state.height)
                }
            }
            style:display=move || {
                if *state.is_opened.read() {
                    "block"
                } else {
                    "none"
                }
            }
            style:top=move || {
                if *state.is_maximized.read() {
                    "110px".to_string()
                } else {
                    format!("{}px", state.y.read())
                }
            }
            style:left=move || {
                if *state.is_maximized.read() {
                    "10px".to_string()
                } else {
                    format!("{}px", state.x.read())
                }
            }
            style:z-index=move || {
                if state.drag.read().is_some() || *state.is_maximized.read() {
                    "99"
                } else {
                    "0"
                }
            }
        >
            <div
                class="window-title"
                on:mousedown=start_drag
                on:mousemove=drag_window
                on:mouseup=stop_drag
            >
                <h1>{state.title}</h1>
                <div class="window-title-buttons">
                    <div
                        class="window-title-button minimize"
                        on:click=is_minimized_toggle
                    >
                        "−"
                    </div>
                    <div
                        class="window-title-button maximize"
                        on:click=is_maximized_toggle
                    >
                        "□"
                    </div>
                    <div
                        class="window-title-button close"
                        on:click=is_opened_toggle
                    >
                        "×"
                    </div>
                </div>
            </div>
            <div class="window-content">{state.content}</div>
        </div>
        <style>
            {"body {"}
                {move || body_style.get()}
            {"}"}
        </style>
    }
}
