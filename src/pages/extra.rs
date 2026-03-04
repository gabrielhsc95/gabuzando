use yew::prelude::*;
use crate::components::window::WindowProps;

#[function_component(TerminalWindow)]
pub fn terminal_window() -> Html {
    html! {
        <div class="terminal-window">
            <div class="terminal-output">
                <p class="terminal-text">{"Welcome to Gabuzando Terminal."}</p>
                <p class="terminal-text">{"Type 'help' for a list of commands."}</p>
            </div>
            <div class="terminal-input-row">
                <span class="terminal-prompt">{"> "}</span>
                <input 
                    type="text" 
                    class="terminal-input"
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
