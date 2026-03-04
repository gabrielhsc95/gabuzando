use yew::prelude::*;
use log::info;
use std::collections::HashMap;

use crate::components::window::{WindowProps, WindowWidget, WindowState};

#[derive(Properties, PartialEq)]
pub struct WindowManagerProps {
    pub windows: Option<Vec<WindowProps>>,
}

#[derive(PartialEq, Clone)]
struct ContextMenuState {
    x: i32,
    y: i32,
}

#[function_component(WindowManager)]
pub fn window_manager(props: &WindowManagerProps) -> Html {
    // Store windows with a unique ID to use as key
    #[derive(Clone, PartialEq)]
    struct KeyedWindow {
        id: usize,
        props: WindowProps,
    }

    let extra_windows = use_state(|| Vec::<KeyedWindow>::new());
    let next_id = use_state(|| 0);
    let context_menu = use_state(|| None::<ContextMenuState>);
    let hovered_category = use_state(|| None::<String>);

    let window_states = use_state(|| HashMap::<String, WindowState>::new());

    let all_windows = {
        // We need to mix props.windows (static) and extra_windows (dynamic).
        // For static windows, we can generate stable keys based on index if they don't change often.
        // Or better, we treat them as having negative IDs or distinct keys.
        let mut windows = Vec::new();
        if let Some(static_windows) = &props.windows {
            for (i, w) in static_windows.iter().enumerate() {
                windows.push((format!("static-{}", i), w.clone()));
            }
        }
        for kw in extra_windows.iter() {
            windows.push((format!("dynamic-{}", kw.id), kw.props.clone()));
        }
        windows
    };

    let on_window_state_change = {
        let window_states = window_states.clone();
        Callback::from(move |(key, new_state): (String, WindowState)| {
            let mut states = (*window_states).clone();
            states.insert(key, new_state);
            window_states.set(states);
        })
    };

    let close_menu = {
        let context_menu = context_menu.clone();
        let hovered_category = hovered_category.clone();
        Callback::from(move |_| {
            context_menu.set(None);
            hovered_category.set(None);
        })
    };

    let on_background_click = {
        let close_menu = close_menu.clone();
        let context_menu = context_menu.clone();
        Callback::from(move |e: MouseEvent| {
            if context_menu.is_some() {
                close_menu.emit(());
                return;
            }
            context_menu.set(Some(ContextMenuState {
                x: e.client_x(),
                y: e.client_y(),
            }));
        })
    };

    let open_window = {
        let extra_windows = extra_windows.clone();
        let next_id = next_id.clone();
        let close_menu = close_menu.clone();
        Callback::from(move |window: WindowProps| {
            let mut windows = (*extra_windows).clone();
            let count = windows.len() as f64;
            let mut w = window.clone();
            
            // Offset logic
            w.x += (count + 1.0) * 20.0; // Increased offset (20px) to be more visible
            w.y += (count + 1.0) * 20.0;
            
            // Simple bound check to keep it somewhat on screen (optional)
            if w.x > 80.0 { w.x = 20.0; }
            if w.y > 80.0 { w.y = 20.0; }

            let id = *next_id;
            next_id.set(id + 1);
            
            windows.push(KeyedWindow { id, props: w });
            extra_windows.set(windows);
            close_menu.emit(());
        })
    };

    let handle_mouseenter = {
        let hovered_category = hovered_category.clone();
        Callback::from(move |category: String| {
            hovered_category.set(Some(category));
        })
    };

    let to_title_case = |s: &str| -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    };

    info!("WindowManager rendering with {} static + {} dynamic windows", 
        props.windows.as_ref().map(|v| v.len()).unwrap_or(0), 
        extra_windows.len()
    );

    html! {
        <div 
            style="width: 100vw; height: 100vh; position: fixed; top: 0; left: 0; overflow: hidden;"
            onclick={on_background_click}
        >
            { for all_windows.iter().map(|(key, w_props)| {

                let state = window_states.get(key).cloned();
                
                let on_state_change = {
                    let on_window_state_change = on_window_state_change.clone();
                    let key = key.clone();
                    Callback::from(move |new_state| {
                        on_window_state_change.emit((key.clone(), new_state));
                    })
                };

                html! { 
                    <div key={key.clone()} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <WindowWidget state={state} on_state_change={on_state_change} ..w_props.clone() /> 
                    </div>
                }
            }) }

            if let Some(menu_state) = &*context_menu {
                <div 
                    style={format!("position: absolute; top: {}px; left: {}px; background: #252525; border: 1px solid #444; box-shadow: 0 4px 6px rgba(0,0,0,0.3); z-index: 1000; min-width: 160px; color: #eeeeee; border-radius: 4px;", menu_state.y, menu_state.x)}
                    onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                >
                    { for ["home", "cv", "projects", "blog", "about", "extra"].iter().map(|cat| {
                        let cat = (*cat).to_string();
                        let is_hovered = hovered_category.as_ref() == Some(&cat);
                        
                        html! {
                            <div 
                                class="menu-item" 
                                onmouseenter={
                                    let handle_mouseenter = handle_mouseenter.clone();
                                    let cat = cat.clone();
                                    move |_| handle_mouseenter.emit(cat.clone())
                                }
                                style="padding: 8px 12px; cursor: pointer; display: flex; justify-content: space-between; position: relative;"
                            >
                                {to_title_case(&cat)}
                                <span>{"›"}</span>
                                
                                { if is_hovered {
                                    html! {
                                        <div style="position: absolute; left: 100%; top: 0; background: #252525; border: 1px solid #444; box-shadow: 0 4px 6px rgba(0,0,0,0.3); min-width: 160px; color: #eeeeee; border-radius: 4px;">
                                            {
                                                match cat.as_str() {
                                                    "home" => html! {
                                                        <div class="menu-item" onclick={
                                                            let open_window = open_window.clone();
                                                            move |_| open_window.emit(crate::pages::home::get_greeting_window())
                                                        }>{"Greeting"}</div>
                                                    },
                                                    "cv" => html! {
                                                        <>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::cv::get_experience_window())
                                                            }>{"Experience"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::cv::get_education_window())
                                                            }>{"Education"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::cv::get_skills_window())
                                                            }>{"Skills"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::cv::get_additional_info_window())
                                                            }>{"Additional Info"}</div>
                                                        </>
                                                    },
                                                    "about" => html! {
                                                        <>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::about::get_me_window())
                                                            }>{"Me"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::about::get_photo_window())
                                                            }>{"Photo"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::about::get_countries_window())
                                                            }>{"Countries"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::about::get_contact_window())
                                                            }>{"Contact"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::about::get_why_window())
                                                            }>{"Why Gabuzando"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::about::get_quote_window())
                                                            }>{"Quote"}</div>
                                                        </>
                                                    },
                                                    "projects" => html! {
                                                        <>
                                                            { for [
                                                                "crustacean-capital", "gabuzando", "cinema-city", "logfire-rust", 
                                                                "tictactoe", "live-bootcamp-project", "mentor_xmercury", "mentor_xlunar", "placeholder_1"
                                                            ].iter().map(|id| {
                                                                let id = (*id).to_string();
                                                                html! {
                                                                    <div class="menu-item" onclick={
                                                                        let open_window = open_window.clone();
                                                                        let id = id.clone();
                                                                        move |_| open_window.emit(crate::pages::projects::get_project_window(&id))
                                                                    }>{id}</div>
                                                                }
                                                            })}
                                                        </>
                                                    },
                                                    "blog" => html! {
                                                        <>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::blog::get_post_window(None))
                                                            }>{"Latest Post"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::blog::get_history_window())
                                                            }>{"History"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::blog::get_best_window())
                                                            }>{"Best"}</div>
                                                            <div class="menu-item" onclick={
                                                                let open_window = open_window.clone();
                                                                move |_| open_window.emit(crate::pages::blog::get_random_window())
                                                            }>{"Random"}</div>
                                                        </>
                                                    },
                                                    "extra" => html! {
                                                        <div class="menu-item" onclick={
                                                            let open_window = open_window.clone();
                                                            move |_| open_window.emit(crate::pages::extra::get_terminal_window())
                                                        }>{"Terminal"}</div>
                                                    },
                                                    _ => html! {}
                                                }
                                            }
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                        }
                    })}
                </div>
            }

            <style>
                { "
                    .menu-item, .menu-item-back {
                        padding: 8px 12px;
                        cursor: pointer;
                        border-bottom: 1px solid #333;
                        transition: background-color 0.2s;
                    }
                    .menu-item:last-child {
                        border-bottom: none;
                    }
                    .menu-item:hover, .menu-item-back:hover {
                        background-color: #333;
                        color: #fff;
                    }
                " }
            </style>
        </div>
    }
}
