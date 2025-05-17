use leptos::prelude::*;
pub struct WindowState<Content: IntoView + 'static> {
    title: String,
    content: Content,
    x: i32,
    default_x: i32,
    y: i32,
    default_y: i32,
    width: i32,
    default_width: i32,
    height: i32,
    default_height: i32,
    is_opened: RwSignal<bool>,
    is_minimized: RwSignal<bool>,
}

impl<Content: IntoView + 'static> WindowState<Content> {
    pub fn new(title: String, content: Content, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            title,
            content,
            x,
            default_x: x,
            y,
            default_y: y,
            width,
            default_width: width,
            height,
            default_height: height,
            is_opened: RwSignal::new(true),
            is_minimized: RwSignal::new(false),
        }
    }
}

#[component]
pub fn WindowWidget<Content: IntoView + 'static>(state: WindowState<Content>) -> impl IntoView {
    let is_opened_toggle = move |_| {
        state.is_opened.update(|open: &mut bool| *open = !*open);
    };

    let is_minimized_toggle = move |_| {
        state
            .is_minimized
            .update(|minimize: &mut bool| *minimize = !*minimize);
    };

    view! {
        <div
            class="window"
            style:width=move || format!("{}px", state.width)
            style:height=move || {if *state.is_minimized.read() {"37px".to_string()} else {format!("{}px", state.height)}}
            style:display=move || { if *state.is_opened.read() {"block"} else {"none"}}
            style:top=move || format!("{}px", state.x)
            style:left=move || format!("{}px", state.y)
        >
            <div class="window-title">
                <h1>{state.title}</h1>
                <div class="window-title-buttons">
                    <div
                        class="window-title-button minimize"
                        on:click=is_minimized_toggle
                    >
                        "−"
                    </div>
                    <div class="window-title-button maximize">"□"</div>
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
    }
}
