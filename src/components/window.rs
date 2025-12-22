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

#[derive(Clone, Copy, Debug, PartialEq)]
enum ResizeDirection {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[function_component(WindowWidget)]
pub fn window_widget(props: &WindowProps) -> Html {
    let div_ref = use_node_ref();
    let body_style = use_state(|| String::from(""));
    let drag = use_state(|| None::<(i32, i32)>);
    let resizing = use_state(|| None::<ResizeDirection>);
    let resize_start_pos = use_state(|| None::<(i32, i32)>);
    
    let x = use_state(|| props.x);
    let y = use_state(|| props.y);
    let width = use_state(|| props.width);
    let height = use_state(|| props.height);
    
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
            e.stop_propagation(); // Prevent triggering resize if clicking title bar near edge
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
        let x_for_closure = x.clone();
        let y_for_closure = y.clone();
        let is_minimized = is_minimized.clone();
        let buffer = props.buffer;
        // Use current width/height state instead of props
        let current_width = width.clone();
        let current_height = height.clone();
        
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
                            
                            let buffer_x_p = (buffer_px / width_px) * 100.0;
                            let buffer_y_p = (buffer_px / height_px) * 100.0;

                            let current_w_p = if *is_minimized { 15.0 } else { *current_width };
                            let current_h_p = if *is_minimized { 
                                (37.0 / height_px) * 100.0 
                            } else { 
                                *current_height 
                            };

                            let min_x = buffer_x_p;
                            let max_x = 100.0 - buffer_x_p - current_w_p;
                            
                            let min_y = (header_h_px / height_px * 100.0) + buffer_y_p;
                            let max_y = 100.0 - (footer_h_px / height_px * 100.0) - buffer_y_p - current_h_p;

                            if new_x < min_x { new_x = min_x; }
                            if new_x > max_x { new_x = max_x; }
                            if new_y < min_y { new_y = min_y; }
                            if new_y > max_y { new_y = max_y; }

                            drag.set(Some((mouse_x, mouse_y)));
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

    // Resize handlers
    let start_resize = |direction: ResizeDirection| {
        let resizing = resizing.clone();
        let resize_start_pos = resize_start_pos.clone();
        let body_style = body_style.clone();
        
        Callback::from(move |e: PointerEvent| {
            e.prevent_default();
            e.stop_propagation();
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<Element>() {
                    let _ = element.set_pointer_capture(e.pointer_id());
                }
            }
            resizing.set(Some(direction));
            resize_start_pos.set(Some((e.client_x(), e.client_y())));
            body_style.set("user-select: none;".to_string());
        })
    };

    let resize_move = {
        let resizing = resizing.clone();
        let resize_start_pos = resize_start_pos.clone();
        let current_x = x.clone();
        let current_y = y.clone();
        let current_width = width.clone();
        let current_height = height.clone();
        let x_setter = x.clone();
        let y_setter = y.clone();
        let width_setter = width.clone();
        let height_setter = height.clone();
        let buffer = props.buffer;

        Callback::from(move |e: PointerEvent| {
            if let Some(direction) = *resizing {
                if let Some((start_x, start_y)) = *resize_start_pos {
                    let mouse_x = e.client_x();
                    let mouse_y = e.client_y();
                    let delta_x_px = mouse_x - start_x;
                    let delta_y_px = mouse_y - start_y;

                    if let Some(window) = web_sys::window() {
                        if let (Ok(inner_width_val), Ok(inner_height_val)) = (window.inner_width(), window.inner_height()) {
                            if let (Some(viewport_w), Some(viewport_h)) = (inner_width_val.as_f64(), inner_height_val.as_f64()) {
                                
                                let delta_x_p = (delta_x_px as f64 / viewport_w) * 100.0;
                                let delta_y_p = (delta_y_px as f64 / viewport_h) * 100.0;

                                // Constraints constants
                                let buffer_px = buffer;
                                let header_h_px = 85.0;
                                let footer_h_px = 30.0;
                                
                                let buffer_x_p = (buffer_px / viewport_w) * 100.0;
                                let buffer_y_p = (buffer_px / viewport_h) * 100.0;
                                let header_h_p = (header_h_px / viewport_h) * 100.0;
                                let footer_h_p = (footer_h_px / viewport_h) * 100.0;

                                let min_x_bound = buffer_x_p;
                                let max_x_bound = 100.0 - buffer_x_p;
                                let min_y_bound = header_h_p + buffer_y_p;
                                let max_y_bound = 100.0 - footer_h_p - buffer_y_p;

                                let mut new_x = *current_x;
                                let mut new_y = *current_y;
                                let mut new_w = *current_width;
                                let mut new_h = *current_height;

                                match direction {
                                    ResizeDirection::Right => {
                                        new_w = *current_width + delta_x_p;
                                        // Clamp Right
                                        if new_x + new_w > max_x_bound {
                                            new_w = max_x_bound - new_x;
                                        }
                                    },
                                    ResizeDirection::Bottom => {
                                        new_h = *current_height + delta_y_p;
                                        // Clamp Bottom
                                        if new_y + new_h > max_y_bound {
                                            new_h = max_y_bound - new_y;
                                        }
                                    },
                                    ResizeDirection::Left => {
                                        new_x = *current_x + delta_x_p;
                                        new_w = *current_width - delta_x_p;
                                        // Clamp Left
                                        if new_x < min_x_bound {
                                            let diff = min_x_bound - new_x;
                                            new_x = min_x_bound;
                                            new_w -= diff;
                                        }
                                    },
                                    ResizeDirection::Top => {
                                        new_y = *current_y + delta_y_p;
                                        new_h = *current_height - delta_y_p;
                                        // Clamp Top
                                        if new_y < min_y_bound {
                                            let diff = min_y_bound - new_y;
                                            new_y = min_y_bound;
                                            new_h -= diff;
                                        }
                                    },
                                    ResizeDirection::BottomRight => {
                                        new_w = *current_width + delta_x_p;
                                        new_h = *current_height + delta_y_p;
                                        if new_x + new_w > max_x_bound { new_w = max_x_bound - new_x; }
                                        if new_y + new_h > max_y_bound { new_h = max_y_bound - new_y; }
                                    },
                                    ResizeDirection::BottomLeft => {
                                        new_x = *current_x + delta_x_p;
                                        new_w = *current_width - delta_x_p;
                                        new_h = *current_height + delta_y_p;
                                        if new_x < min_x_bound {
                                            let diff = min_x_bound - new_x;
                                            new_x = min_x_bound;
                                            new_w -= diff;
                                        }
                                        if new_y + new_h > max_y_bound { new_h = max_y_bound - new_y; }
                                    },
                                    ResizeDirection::TopRight => {
                                        new_y = *current_y + delta_y_p;
                                        new_h = *current_height - delta_y_p;
                                        new_w = *current_width + delta_x_p;
                                        if new_y < min_y_bound {
                                            let diff = min_y_bound - new_y;
                                            new_y = min_y_bound;
                                            new_h -= diff;
                                        }
                                        if new_x + new_w > max_x_bound { new_w = max_x_bound - new_x; }
                                    },
                                    ResizeDirection::TopLeft => {
                                        new_x = *current_x + delta_x_p;
                                        new_w = *current_width - delta_x_p;
                                        new_y = *current_y + delta_y_p;
                                        new_h = *current_height - delta_y_p;
                                        if new_x < min_x_bound {
                                            let diff = min_x_bound - new_x;
                                            new_x = min_x_bound;
                                            new_w -= diff;
                                        }
                                        if new_y < min_y_bound {
                                            let diff = min_y_bound - new_y;
                                            new_y = min_y_bound;
                                            new_h -= diff;
                                        }
                                    },
                                }

                                // Minimum size constraints (e.g. 5% width, 5% height or pixel equivalent)
                                let min_w_p = (300.0 / viewport_w) * 100.0; // 300px min width
                                let min_h_p = (150.0 / viewport_h) * 100.0; // 150px min height
                                
                                if new_w < min_w_p { 
                                    if matches!(direction, ResizeDirection::Left | ResizeDirection::TopLeft | ResizeDirection::BottomLeft) {
                                        new_x = *current_x + (*current_width - min_w_p);
                                    }
                                    new_w = min_w_p; 
                                }
                                if new_h < min_h_p {
                                    if matches!(direction, ResizeDirection::Top | ResizeDirection::TopLeft | ResizeDirection::TopRight) {
                                        new_y = *current_y + (*current_height - min_h_p);
                                    }
                                    new_h = min_h_p; 
                                }

                                x_setter.set(new_x);
                                y_setter.set(new_y);
                                width_setter.set(new_w);
                                height_setter.set(new_h);
                                
                                resize_start_pos.set(Some((mouse_x, mouse_y)));
                            }
                        }
                    }
                }
            }
        })
    };

    let stop_resize = {
        let resizing = resizing.clone();
        let resize_start_pos = resize_start_pos.clone();
        let body_style = body_style.clone();
        Callback::from(move |e: PointerEvent| {
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<Element>() {
                    let _ = element.release_pointer_capture(e.pointer_id());
                }
            }
            resizing.set(None);
            resize_start_pos.set(None);
            body_style.set("".to_string());
        })
    };

    let width_style = if *is_maximized {
        "98%".to_string()
    } else {
        format!("{}%", is_minimized.then(|| 15.0).unwrap_or(*width))
    };

    let height_style = if *is_minimized {
        "37px".to_string()
    } else if *is_maximized {
        "87%".to_string()
    } else {
        format!("{}%", *height)
    };

    let display_style = if *is_opened { "flex" } else { "none" };

    let top_style = if *is_maximized {
        "10%".to_string()
    } else {
        format!("{}%", *y)
    };

    let left_style = if *is_maximized {
        "1%".to_string()
    } else {
        format!("{}%", *x)
    };

    let z_index_style = if drag.is_some() || resizing.is_some() || *is_maximized {
        "99"
    } else {
        "0"
    };

    let body_css = format!("body {{ {} }}", *body_style);

    // Helper to generate handle HTML
    let make_handle = |class: &str, dir: ResizeDirection| {
        let start = start_resize(dir);
        let move_handler = resize_move.clone();
        let stop_handler = stop_resize.clone();
        html! {
            <div class={format!("resize-handle {}", class)}
                 onpointerdown={start}
                 onpointermove={move_handler}
                 onpointerup={stop_handler}
            />
        }
    };

    html! {
        <>
            <div
                class="window"
                ref={div_ref}
                style={format!("width:{}; height:{}; display:{}; top:{}; left:{}; z-index:{};",
                    width_style, height_style, display_style, top_style, left_style, z_index_style
                )}
            >
                { if !*is_minimized && !*is_maximized {
                    html! {
                        <>
                            { make_handle("top", ResizeDirection::Top) }
                            { make_handle("bottom", ResizeDirection::Bottom) }
                            { make_handle("left", ResizeDirection::Left) }
                            { make_handle("right", ResizeDirection::Right) }
                            { make_handle("top-left", ResizeDirection::TopLeft) }
                            { make_handle("top-right", ResizeDirection::TopRight) }
                            { make_handle("bottom-left", ResizeDirection::BottomLeft) }
                            { make_handle("bottom-right", ResizeDirection::BottomRight) }
                        </>
                    }
                } else { html! {} } }

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
