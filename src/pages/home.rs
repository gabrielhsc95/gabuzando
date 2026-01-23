use yew::prelude::*;
use crate::components::window::WindowProps;
use rand::prelude::*;
use crate::components::loading::Loading;
use crate::hooks::{use_fetch, FetchState};
use crate::types::{GreetingsList, SimpleTextContent};
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

pub fn get_greeting_window() -> WindowProps {
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
        state: None,
        on_state_change: Callback::noop(),
    }
}

pub fn get_home_windows() -> Vec<WindowProps> {
    vec![
        // Cell 1 + 4: CV (Experience)
        {
             let mut w = crate::pages::cv::get_experience_window();
             w.x = 1.0; w.y = 10.0; w.width = 31.6; w.height = 55.6;
             w
        },
        // Cell 2 + 3: About (Me)
        {
            let mut w = crate::pages::about::get_me_window();
            w.x = 34.6; w.y = 10.0; w.width = 64.2; w.height = 27.3;
            w
        },
        // Cell 5: Project (TicTacToe)
        {
            let mut w = crate::pages::projects::get_project_window("tictactoe");
            w.x = 34.6; w.y = 39.3; w.width = 30.6; w.height = 26.3;
            w
        },
        // Cell 6: Blog (Best)
         {
             let mut w = crate::pages::blog::get_best_window();
             w.x = 67.2; w.y = 39.3; w.width = 31.6; w.height = 26.3;
             w
         },
        // Cell 7: Project (Mentor XMercury)
        {
            let mut w = crate::pages::projects::get_project_window("mentor_xmercury");
            w.x = 1.0; w.y = 67.6; w.width = 31.6; w.height = 27.3;
            w
        },
        // Cell 8: Project (Mentor XLunar)
        {
            let mut w = crate::pages::projects::get_project_window("mentor_xlunar");
            w.x = 34.6; w.y = 67.6; w.width = 30.6; w.height = 27.3;
             w
        },
        // Cell 9: Home (Greetings)
        get_greeting_window(),
    ]
}
