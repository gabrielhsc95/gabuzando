use yew::prelude::*;
use web_sys::{HtmlInputElement, MutationObserver, MutationObserverInit, wasm_bindgen::JsCast, wasm_bindgen::closure::Closure, js_sys};
use crate::components::window::WindowProps;
use crate::pages::about::{MeLoader as TerminalMeLoader, CountriesLoader as TerminalCountriesLoader, ContactLoader as TerminalContactLoader, WhyLoader as TerminalWhyLoader, QuoteLoader as TerminalQuoteLoader};
use crate::pages::home::GreetingsLoader as TerminalGreetingsLoader;
use crate::pages::cv::{ExperienceLoader as TerminalExperienceLoader, EducationLoader as TerminalEducationLoader, SkillsLoader as TerminalSkillsLoader, AdditionalInfoLoader as TerminalAdditionalInfoLoader};
use crate::pages::projects::ProjectLoader as TerminalProjectLoader;
use crate::pages::blog::{BlogLoader as TerminalBlogLoader, BlogMode};
#[derive(Clone, PartialEq)]
enum TerminalOutput {
    Text(String),
    Command(String),
    System(String),
    AboutMeLoader,
    AboutCountriesLoader,
    AboutContactLoader,
    AboutWhyLoader,
    AboutQuoteLoader,
    HomeGreetingsLoader,
    CvExperienceLoader,
    CvEducationLoader,
    CvSkillsLoader,
    CvAdditionalInfoLoader,
    ProjectLoader(String),
    BlogLoader(BlogMode, Option<String>),
}

#[function_component(TerminalWindow)]
pub fn terminal_window() -> Html {
    let history = use_state(|| vec![
        TerminalOutput::System("Welcome to Gabuzando Terminal.".to_string()),
        TerminalOutput::System("Type 'help' for a list of commands.".to_string()),
    ]);
    let input_value = use_state(|| String::new());
    let input_ref = use_node_ref();

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                input_value.set(target.value());
            }
        })
    };

    let on_keydown = {
        let history = history.clone();
        let input_value = input_value.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let cmd = (*input_value).trim().to_string();
                if cmd.is_empty() {
                    return;
                }

                let mut new_history = (*history).clone();
                new_history.push(TerminalOutput::Command(cmd.clone()));
                
                // Parse command
                let mut parts = cmd.split_whitespace();
                if let Some(main_cmd) = parts.next() {
                    let args: Vec<String> = parts.map(|s| s.replace("_", "-")).collect();
                    match main_cmd {
                        "help" => {
                            new_history.push(TerminalOutput::Text("Available commands:".to_string()));
                            new_history.push(TerminalOutput::Text("  about [me | photo | countries | contact | why-gabuzando | quote]".to_string()));
                            new_history.push(TerminalOutput::Text("  home [greetings]".to_string()));
                            new_history.push(TerminalOutput::Text("  cv [experience | education | skills | additional-info]".to_string()));
                            new_history.push(TerminalOutput::Text("  projects [crustacean-capital | gabuzando | cinema-city | logfire-rust | ...]".to_string()));
                            new_history.push(TerminalOutput::Text("  blog [latest | history | best | random]".to_string()));
                            new_history.push(TerminalOutput::Text("  clear".to_string()));
                        },
                        "clear" => {
                            new_history.clear();
                        },
                        "about" => {
                            if args.is_empty() {
                                new_history.push(TerminalOutput::Text("Usage: about [argument]".to_string()));
                                new_history.push(TerminalOutput::Text("Available about arguments: me, photo, countries, contact, why-gabuzando, quote".to_string()));
                            } else {
                                match args[0].as_str() {
                                    "me" => new_history.push(TerminalOutput::AboutMeLoader),
                                    "countries" => new_history.push(TerminalOutput::AboutCountriesLoader),
                                    "contact" => new_history.push(TerminalOutput::AboutContactLoader),
                                    "why-gabuzando" => new_history.push(TerminalOutput::AboutWhyLoader),
                                    "quote" => new_history.push(TerminalOutput::AboutQuoteLoader),
                                    "photo" => new_history.push(TerminalOutput::Text("[Photo rendering not supported in terminal]".to_string())),
                                    _ => new_history.push(TerminalOutput::Text(format!("Unknown about argument: {}", args[0]))),
                                }
                            }
                        },
                        "home" => {
                            if args.is_empty() {
                                new_history.push(TerminalOutput::Text("Usage: home [greetings]".to_string()));
                            } else {
                                match args[0].as_str() {
                                    "greetings" => new_history.push(TerminalOutput::HomeGreetingsLoader),
                                    _ => new_history.push(TerminalOutput::Text(format!("Unknown home argument: {}", args[0]))),
                                }
                            }
                        },
                        "cv" => {
                            if args.is_empty() {
                                new_history.push(TerminalOutput::Text("Usage: cv [experience | education | skills | additional-info]".to_string()));
                            } else {
                                match args[0].as_str() {
                                    "experience" => new_history.push(TerminalOutput::CvExperienceLoader),
                                    "education" => new_history.push(TerminalOutput::CvEducationLoader),
                                    "skills" => new_history.push(TerminalOutput::CvSkillsLoader),
                                    "additional-info" => new_history.push(TerminalOutput::CvAdditionalInfoLoader),
                                    _ => new_history.push(TerminalOutput::Text(format!("Unknown cv argument: {}", args[0]))),
                                }
                            }
                        },
                        "projects" => {
                            if args.is_empty() {
                                new_history.push(TerminalOutput::Text("Usage: projects [id]".to_string()));
                                new_history.push(TerminalOutput::Text("Try: crustacean-capital, gabuzando, cinema-city, logfire-rust, tictactoe, live-bootcamp-project, mentor-xmercury, mentor-xlunar".to_string()));
                            } else {
                                new_history.push(TerminalOutput::ProjectLoader(args[0].clone()));
                            }
                        },
                        "blog" => {
                            if args.is_empty() {
                                new_history.push(TerminalOutput::Text("Usage: blog [latest | history | best | random | <post-slug>]".to_string()));
                            } else {
                                match args[0].as_str() {
                                    "latest" => new_history.push(TerminalOutput::BlogLoader(BlogMode::Post, None)),
                                    "history" => new_history.push(TerminalOutput::BlogLoader(BlogMode::History, None)),
                                    "best" => new_history.push(TerminalOutput::BlogLoader(BlogMode::Best, None)),
                                    "random" => new_history.push(TerminalOutput::BlogLoader(BlogMode::Random, None)),
                                    slug => new_history.push(TerminalOutput::BlogLoader(BlogMode::Post, Some(slug.to_string()))),
                                }
                            }
                        },
                        _ => {
                            new_history.push(TerminalOutput::Text(format!("Command not found: {}", main_cmd)));
                        }
                    }
                }

                history.set(new_history);
                input_value.set(String::new());
            }
        })
    };

    let terminal_output_ref = use_node_ref();

    {
        let terminal_output_ref = terminal_output_ref.clone();
        use_effect_with(terminal_output_ref, move |node_ref| {
            let mut observer_handle = None;
            
            if let Some(element) = node_ref.cast::<web_sys::Element>() {
                let target = element.clone();
                let cb = Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(
                    move |_mutations: js_sys::Array, _observer: web_sys::MutationObserver| {
                        target.set_scroll_top(target.scroll_height());
                    }
                );
                
                if let Ok(observer) = MutationObserver::new(cb.as_ref().unchecked_ref()) {
                    let init = MutationObserverInit::new();
                    init.set_child_list(true);
                    init.set_subtree(true);
                    
                    if observer.observe_with_options(&element, &init).is_ok() {
                        observer_handle = Some((observer, cb));
                    }
                }
            }
            
            move || {
                if let Some((observer, _cb)) = observer_handle {
                    observer.disconnect();
                }
            }
        });
    }

    let last_cmd_idx = history.iter().rposition(|item| matches!(item, TerminalOutput::Command(_)));

    html! {
        <div class="terminal-window" onclick={Callback::from(move |_| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(input) = document.query_selector(".terminal-input").unwrap_or(None) {
                        if let Ok(html_input) = input.dyn_into::<web_sys::HtmlElement>() {
                            let _ = html_input.focus();
                        }
                    }
                }
            }
        })}>
            <div class="terminal-output" ref={terminal_output_ref}>
                { for history.iter().enumerate().map(|(idx, item)| match item {
                    TerminalOutput::System(text) => html! { <p class="terminal-text" style="color: #aaa;">{ text }</p> },
                    TerminalOutput::Command(cmd) => {
                        let id = if Some(idx) == last_cmd_idx { "latest-command" } else { "" };
                        html! { <p class="terminal-text" id={id}><span class="terminal-prompt">{"> "}</span>{ cmd }</p> }
                    },
                    TerminalOutput::Text(text) => html! { <p class="terminal-text">{ text }</p> },
                    TerminalOutput::AboutMeLoader => html! { <div class="terminal-text"><TerminalMeLoader /></div> },
                    TerminalOutput::AboutCountriesLoader => html! { <div class="terminal-text"><TerminalCountriesLoader /></div> },
                    TerminalOutput::AboutContactLoader => html! { <div class="terminal-text"><TerminalContactLoader /></div> },
                    TerminalOutput::AboutWhyLoader => html! { <div class="terminal-text"><TerminalWhyLoader /></div> },
                    TerminalOutput::AboutQuoteLoader => html! { <div class="terminal-text"><TerminalQuoteLoader /></div> },
                    TerminalOutput::HomeGreetingsLoader => html! { <div class="terminal-text"><TerminalGreetingsLoader /></div> },
                    TerminalOutput::CvExperienceLoader => html! { <div class="terminal-text"><TerminalExperienceLoader /></div> },
                    TerminalOutput::CvEducationLoader => html! { <div class="terminal-text"><TerminalEducationLoader /></div> },
                    TerminalOutput::CvSkillsLoader => html! { <div class="terminal-text"><TerminalSkillsLoader /></div> },
                    TerminalOutput::CvAdditionalInfoLoader => html! { <div class="terminal-text"><TerminalAdditionalInfoLoader /></div> },
                    TerminalOutput::ProjectLoader(id) => html! { <div class="terminal-text"><TerminalProjectLoader id={id.clone()} /></div> },
                    TerminalOutput::BlogLoader(mode, slug) => html! { <div class="terminal-text"><TerminalBlogLoader mode={mode.clone()} slug={slug.clone()} /></div> },
                }) }
            </div>
            <div class="terminal-input-row">
                <span class="terminal-prompt">{"> "}</span>
                <input 
                    type="text" 
                    class="terminal-input"
                    value={(*input_value).clone()}
                    oninput={on_input}
                    onkeydown={on_keydown}
                    ref={input_ref}
                    autofocus=true 
                />
            </div>
        </div>
    }
}

pub fn get_terminal_window() -> WindowProps {
    WindowProps {
        title: AttrValue::from("extra/terminal"),
        content: yew::html::ChildrenRenderer::new(vec![html! {
            <TerminalWindow />
        }]),
        x: 15.0,
        y: 15.0,
        width: 70.0,
        height: 60.0,
        buffer: 20.0,
        no_padding: true,
        ..Default::default()
    }
}
