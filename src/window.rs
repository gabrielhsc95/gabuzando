use web_sys::{Element, PointerEvent, wasm_bindgen::JsCast};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct WindowProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub content: Children,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[prop_or(10.0)]
    pub buffer: f64,
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
        let is_minimized = is_minimized.clone();
        let buffer = props.buffer;
        let props_width = props.width;
        let props_height = props.height;
        
        Callback::from(move |e: PointerEvent| {
            if let Some((initial_x, initial_y)) = *drag {
                let mouse_x = e.client_x();
                let mouse_y = e.client_y();
                let delta_x_px = mouse_x - initial_x;
                let delta_y_px = mouse_y - initial_y;
                
                if let Some(window) = web_sys::window() {
                    if let (Ok(inner_width), Ok(inner_height)) = (window.inner_width(), window.inner_height()) {
                         if let (Some(width_px), Some(height_px)) = (inner_width.as_f64(), inner_height.as_f64()) {
                            let delta_x_p = (delta_x_px as f64 / width_px) * 100.0;
                            let delta_y_p = (delta_y_px as f64 / height_px) * 100.0;
                            
                            let mut new_x = *current_x + delta_x_p;
                            let mut new_y = *current_y + delta_y_p;

                            // Constraints
                            let buffer_px = buffer;
                            let header_h_px = 85.0;
                            let footer_h_px = 30.0;
                            
                            // Calculate buffer in % based on current viewport dimensions
                            let buffer_x_p = (buffer_px / width_px) * 100.0;
                            let buffer_y_p = (buffer_px / height_px) * 100.0;

                            // Calculate current dimensions in %
                            let current_w_p = if *is_minimized { 15.0 } else { props_width };
                            let current_h_p = if *is_minimized { 
                                (37.0 / height_px) * 100.0 
                            } else { 
                                props_height 
                            };

                            let min_x = buffer_x_p;
                            let max_x = 100.0 - buffer_x_p - current_w_p;
                            
                            let min_y = (header_h_px / height_px * 100.0) + buffer_y_p;
                            let max_y = 100.0 - (footer_h_px / height_px * 100.0) - buffer_y_p - current_h_p;

                            // Clamp
                            if new_x < min_x { new_x = min_x; }
                            if new_x > max_x { new_x = max_x; }
                            if new_y < min_y { new_y = min_y; }
                            if new_y > max_y { new_y = max_y; }

                            drag.set(Some((mouse_x, mouse_y)));
                            // Use the cloned state handles
                            x_for_closure.set(new_x);
                            y_for_closure.set(new_y);
                         } 
                    }
                }
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
        format!("{}%", is_minimized.then(|| 15.0).unwrap_or(props.width))
    };

    let height_style = if *is_minimized {
        "37px".to_string()
    } else if *is_maximized {
        "87%".to_string()
    } else {
        format!("{}%", props.height)
    };

    let display_style = if *is_opened { "flex" } else { "none" };

    let top_style = if *is_maximized {
        "10%".to_string() // Rough estimate for top bar spacing in % or specific px if needed, sticking to % as requested
    } else {
        format!("{}%", *y)
    };

    let left_style = if *is_maximized {
        "1%".to_string()
    } else {
        format!("{}%", *x)
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
                     {
                        if !*is_minimized {
                            html! { for props.content.iter() }
                        } else {
                            html! {}
                        }
                     }
                </div>
            </div>
            <style>
                {body_css}
            </style>
        </>
    }
}
