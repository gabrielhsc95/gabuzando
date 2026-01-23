use yew::prelude::*;
use crate::components::window::WindowProps;
use crate::components::loading::Loading;
use crate::hooks::{use_fetch, FetchState};
use crate::types::ProjectsContent;
use crate::components::language_context::use_language;

#[derive(Properties, PartialEq)]
pub struct ProjectLoaderProps {
    pub id: String,
}

#[function_component(ProjectLoader)]
pub fn project_loader(props: &ProjectLoaderProps) -> Html {
    let fetch_state = use_fetch::<ProjectsContent>("/text/projects/projects.json");
    let language = use_language().language;
    let id = props.id.clone();
    
    // We could memoize finding the project, but the list is small.
    // The fetch state holds the list.
    
    match fetch_state {
        FetchState::Success(data) => {
            if let Some(project) = data.projects.iter().find(|p| p.id == id) {
                 html! {
                    <>
                        <div style="display: flex; justify-content: space-between; align-items: baseline;">
                            if let Some(url) = &project.url {
                                <b><a href={url.clone()} target="_blank">{&project.name}</a></b>
                            } else {
                                <b>{&project.name}</b>
                            }
                            <span style="font-size: 0.8em; color: #666; margin-left: 10px;">{format!("Role: {:?}", project.role)}</span>
                        </div>
                        <p>{ project.description.get(language) }</p>
                    </>
                }
            } else {
                html! { <p>{ format!("Project '{}' not found", id) }</p> }
            }
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

pub fn get_project_window(id: &str) -> WindowProps {
    WindowProps {
        title: AttrValue::from(format!("project/{}", id)),
        content: yew::html::ChildrenRenderer::new(vec![html! {
            <ProjectLoader id={id.to_string()} />
        }]),
        x: 1.0, // Default positions, likely to be overridden or managed by window manager placement strategy if possible? 
                // Context menu might need to place it near mouse or center. 
                // For now, let's keep the hardcoded positions in the main getter, 
                // but this factory will return a default one or we should pass x, y etc?
                // The task says "open a new window". 
                // Let's just give it a default satisfying position.
        y: 10.0,
        width: 31.6,
        height: 27.3,
        buffer: 20.0,
        state: None,
        on_state_change: Callback::noop(),
    }
}

pub fn get_projects_windows() -> Vec<WindowProps> {
    vec![
        // Cell 1: Crustacean Capital
        {
            let mut w = get_project_window("crustacean-capital");
            w.x = 1.0; w.y = 10.0; w.width = 31.6; w.height = 27.3;
            w
        },
        // Cell 2: Gabuzando
        {
            let mut w = get_project_window("gabuzando");
            w.x = 34.6; w.y = 10.0; w.width = 30.6; w.height = 27.3;
            w
        },
        // Cell 3: Cinema City
        {
            let mut w = get_project_window("cinema-city");
            w.x = 67.2; w.y = 10.0; w.width = 31.6; w.height = 27.3;
            w
        },
        // Cell 4: Placeholder
        {
            let mut w = get_project_window("logfire-rust");
            w.x = 1.0; w.y = 39.3; w.width = 31.6; w.height = 26.3;
            w
        },
        // Cell 5: Tic Tac Toe
        {
            let mut w = get_project_window("tictactoe");
            w.x = 34.6; w.y = 39.3; w.width = 30.6; w.height = 26.3;
            w
        },
        // Cell 6: Placeholder
        {
            let mut w = get_project_window("live-bootcamp-project");
            w.x = 67.2; w.y = 39.3; w.width = 31.6; w.height = 26.3;
            w
        },
        // Cell 7: Mentor XMercury
        {
            let mut w = get_project_window("mentor_xmercury");
            w.x = 1.0; w.y = 67.6; w.width = 31.6; w.height = 27.3;
            w
        },
        // Cell 8: Mentor XLunar
        {
            let mut w = get_project_window("mentor_xlunar");
            w.x = 34.6; w.y = 67.6; w.width = 30.6; w.height = 27.3;
            w
        },
        // Cell 9: Placeholder
        {
            let mut w = get_project_window("placeholder_1");
            w.x = 67.2; w.y = 67.6; w.width = 31.6; w.height = 27.3;
            w
        },
    ]
}
