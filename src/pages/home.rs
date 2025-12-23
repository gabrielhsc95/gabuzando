use yew::prelude::*;
use crate::components::window::WindowProps;
use rand::prelude::*;
use crate::components::loading::Loading;
use crate::hooks::{use_fetch, FetchState};
use crate::types::{GreetingsList, SimpleTextContent};
use crate::pages::cv::ExperienceLoader;
use crate::pages::projects::ProjectLoader;
use crate::pages::blog::{BlogLoader, BlogMode};
use crate::components::language_context::use_language;


#[function_component(MeLoader)]
fn me_loader() -> Html {
    let fetch_state = use_fetch::<SimpleTextContent>("/text/about/me.json");
    let language = use_language().language;

    match fetch_state {
        FetchState::Success(data) => {
            let div = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            div.set_inner_html(data.text.get(language));
            html! { Html::VRef(div.into()) }
        }
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

#[function_component(GreetingsLoader)]
pub fn greetings_loader() -> Html {
    let fetch_state = use_fetch::<GreetingsList>("/text/home/greetings.json");
    let greeting = use_state(|| None::<String>);

    {
        let greeting = greeting.clone();
        let fetch_state = fetch_state.clone();
        use_effect_with(fetch_state, move |state| {
            if let FetchState::Success(data) = state {
                 let mut rng = rand::thread_rng();
                 if let Some(g) = data.greetings.choose(&mut rng) {
                     greeting.set(Some(g.clone()));
                 }
            }
            || ()
        });
    }

    match fetch_state {
        FetchState::Success(_) => {
            if let Some(text) = &*greeting {
                html! { <p>{ text }</p> }
            } else {
                html! { <Loading /> }
            }
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

pub fn get_home_windows() -> Vec<WindowProps> {
    vec![
        // Cell 1 + 4: CV (Experience)
        WindowProps {
            title: AttrValue::from("cv/experience"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <ExperienceLoader />
            }]),
            x: 1.0,
            y: 10.0,
            width: 31.6,
            height: 55.6,
            buffer: 20.0,
        },
        // Cell 2 + 3: About (Me)
        WindowProps {
            title: AttrValue::from("about/me"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <MeLoader />
            }]),
            x: 34.6,
            y: 10.0,
            width: 64.2,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 5: Project (TicTacToe)
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
        // Cell 6: Blog (Best)
        WindowProps {
            title: AttrValue::from("blog/best"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <BlogLoader mode={BlogMode::Best} />
            }]),
            x: 67.2,
            y: 39.3,
            width: 31.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Cell 7: Project (Mentor XMercury)
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
        // Cell 8: Project (Mentor XLunar)
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
        // Cell 9: Home (Greetings)
        WindowProps {
            title: AttrValue::from("home/greetings"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <GreetingsLoader />
            }]),
            x: 67.2,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
