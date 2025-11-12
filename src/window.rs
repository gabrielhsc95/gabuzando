use web_sys::{Element, PointerEvent, wasm_bindgen::JsCast};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct WindowProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub content: Children,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[function_component(WindowWidget)]
pub fn window_widget(props: &WindowProps) -> Html {
    let div_ref = use_node_ref();
    let body_style = use_state(|| String::from(""));
    let drag = use_state(|| None::<(i32, i32)>);
    let x = use_state(|| props.x);
    let y = use_state(|| props.y);
    let is_minimized = use_state(|| false);
    let is_maximized = use_state(|| false);
    let is_opened = use_state(|| true);

    let is_maximized_toggle = {
        let is_maximized = is_maximized.clone();
        Callback::from(move |_| {
            is_maximized.set(!*is_maximized);
        })
    };

    let is_minimized_toggle = {
        let is_minimized = is_minimized.clone();
        Callback::from(move |_| {
            is_minimized.set(!*is_minimized);
        })
    };

    let is_opened_toggle = {
        let is_opened = is_opened.clone();
        Callback::from(move |_| {
            is_opened.set(!*is_opened);
        })
    };

    let start_drag = {
        let drag = drag.clone();
        let body_style = body_style.clone();
        Callback::from(move |e: PointerEvent| {
            e.prevent_default();
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<Element>() {
                    let _ = element.set_pointer_capture(e.pointer_id());
                }
            }
            drag.set(Some((e.client_x(), e.client_y())));
            body_style.set("user-select: none;".to_string());
        })
    };

    let drag_window = {
        let drag = drag.clone();
        let current_x = x.clone();
        let current_y = y.clone();
        // Clone x and y again for use in the closure.
        let x_for_closure = x.clone();
        let y_for_closure = y.clone();
        Callback::from(move |e: PointerEvent| {
            if let Some((initial_x, initial_y)) = *drag {
                let mouse_x = e.client_x();
                let mouse_y = e.client_y();
                let delta_x = mouse_x - initial_x;
                let delta_y = mouse_y - initial_y;
                drag.set(Some((mouse_x, mouse_y)));
                // Use the cloned state handles
                x_for_closure.set(*current_x + delta_x);
                y_for_closure.set(*current_y + delta_y);
            }
        })
    };

    let stop_drag = {
        let drag = drag.clone();
        let body_style = body_style.clone();
        Callback::from(move |e: PointerEvent| {
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<Element>() {
                    let _ = element.release_pointer_capture(e.pointer_id());
                }
            }
            drag.set(None);
            body_style.set("".to_string());
        })
    };

    let width_style = if *is_maximized {
        "98%".to_string()
    } else {
        format!("{}px", props.width)
    };

    let height_style = if *is_minimized {
        "37px".to_string()
    } else if *is_maximized {
        "87%".to_string()
    } else {
        format!("{}px", props.height)
    };

    let display_style = if *is_opened { "flex" } else { "none" };

    let top_style = if *is_maximized {
        "110px".to_string()
    } else {
        format!("{}px", *y)
    };

    let left_style = if *is_maximized {
        "10px".to_string()
    } else {
        format!("{}px", *x)
    };

    let z_index_style = if drag.is_some() || *is_maximized {
        "99"
    } else {
        "0"
    };

    let body_css = format!("body {{ {} }}", *body_style);

    html! {
        <>
            <div
                class="window"
                ref={div_ref}
                style={format!("width:{}; height:{}; display:{}; top:{}; left:{}; z-index:{};",
                    width_style, height_style, display_style, top_style, left_style, z_index_style
                )}
            >
                <div
                    class="window-title"
                    onpointerdown={start_drag}
                    onpointermove={drag_window}
                    onpointerup={stop_drag}
                >
                    <h1>{&props.title}</h1>
                    <div class="window-title-buttons">
                        <div
                            class="window-title-button minimize"
                            onclick={is_minimized_toggle}
                        >
                            {"−"}
                        </div>
                        <div
                            class="window-title-button maximize"
                            onclick={is_maximized_toggle}
                        >
                            {"□"}
                        </div>
                        <div
                            class="window-title-button close"
                            onclick={is_opened_toggle}
                        >
                            {"×"}
                        </div>
                    </div>
                </div>
                <div class="window-content">
                     { for props.content.iter() }
                </div>
            </div>
            <style>
                {body_css}
            </style>
        </>
    }
}
