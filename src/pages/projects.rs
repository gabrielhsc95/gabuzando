use yew::prelude::*;
use crate::components::window::WindowProps;
use crate::components::loading::Loading;
use crate::hooks::{use_fetch, FetchState};
use crate::types::ProjectsContent;

#[derive(Properties, PartialEq)]
pub struct ProjectLoaderProps {
    pub id: String,
}

#[function_component(ProjectLoader)]
pub fn project_loader(props: &ProjectLoaderProps) -> Html {
    let fetch_state = use_fetch::<ProjectsContent>("/text/projects.json");
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
                        <p>{&project.description}</p>
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

pub fn get_projects_windows() -> Vec<WindowProps> {
    vec![
        // Cell 1: Crustacean Capital
        WindowProps {
            title: AttrValue::from("project/crustacean-capital"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <ProjectLoader id="crustacean-capital" />
            }]),
            x: 1.0,
            y: 10.0,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 2: Gabuzando
        WindowProps {
            title: AttrValue::from("project/gabuzando"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <ProjectLoader id="gabuzando" />
            }]),
            x: 34.6,
            y: 10.0,
            width: 30.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 3: Cinema City
        WindowProps {
            title: AttrValue::from("project/cinema-city"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <ProjectLoader id="cinema-city" />
            }]),
            x: 67.2,
            y: 10.0,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 4: Placeholder
        WindowProps {
            title: AttrValue::from("project/placeholder_1"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <ProjectLoader id="placeholder_1" />
            }]),
            x: 1.0,
            y: 39.3,
            width: 31.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Cell 5: Tic Tac Toe
        WindowProps {
            title: AttrValue::from("project/tictactoe"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                 <ProjectLoader id="tictactoe" />
            }]),
            x: 34.6,
            y: 39.3,
            width: 30.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Cell 6: Placeholder
        WindowProps {
            title: AttrValue::from("project/placeholder_2"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <ProjectLoader id="placeholder_2" />
            }]),
            x: 67.2,
            y: 39.3,
            width: 31.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Cell 7: Mentor XMercury
        WindowProps {
            title: AttrValue::from("project/mentor_xmercury"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                 <ProjectLoader id="mentor_xmercury" />
            }]),
            x: 1.0,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 8: Mentor XLunar
        WindowProps {
            title: AttrValue::from("project/mentor_xlunar"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                 <ProjectLoader id="mentor_xlunar" />
            }]),
            x: 34.6,
            y: 67.6,
            width: 30.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 9: Placeholder
        WindowProps {
            title: AttrValue::from("project/placeholder_3"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <ProjectLoader id="placeholder_3" />
            }]),
            x: 67.2,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
