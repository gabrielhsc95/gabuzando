use yew::prelude::*;
use crate::components::window::WindowProps;
use rand::prelude::*;
use crate::components::loading::Loading;
use crate::hooks::{use_fetch, FetchState};
use crate::types::{SimpleTextContent, GreetingsList};


#[function_component(MeLoader)]
fn me_loader() -> Html {
    let fetch_state = use_fetch::<SimpleTextContent>("/text/about/me.json");

    match fetch_state {
        FetchState::Success(data) => {
            let div = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            div.set_inner_html(&data.text);
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
                <>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Quantitative Researcher (Senior Associate)"}</b></div>
                            <div>{"Aug. 2024 – Present"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States"}</p>
                        <ul>
                            <li>{"Developed an AI financial analyst agent, focusing on named entity recognition and validation."}</li>
                            <li>{"Participated in research discussions to enhance agent capabilities using various LLM models."}</li>
                            <li>{"Understood client engagement to improve on pain points."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Senior Associate)"}</b></div>
                            <div>{"Jan. 2024 - Jul. 2024"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States"}</p>
                        <ul>
                            <li>{"Teach Led in Testing, empowering 4 people to implement new features to an automated testing software"}</li>
                            <li>{"Managed the testing side implementation of two new pricing models."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Associate)"}</b></div>
                            <div>{"Jan. 2023 - Dec. 2023"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States"}</p>
                        <ul>
                            <li>{"Analytical Quality Assurance for financial factor models."}</li>
                            <li>{"Developed and reviewed financial building blocks libraries to facilitate the replication of financial models."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Analyst)"}</b></div>
                            <div>{"Jun. 2021 - Dec. 2022"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States and Budapest, Hungary"}</p>
                        <ul>
                            <li>{"Built an automated testing software for a financial pricing analytics library, and integrated it to the build pipeline."}</li>
                            <li>{"Developed two financial models to estimate financed emissions in accordance with PCAF."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Intern)"}</b></div>
                            <div>{"Sep. 2020 - Jun. 2021"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Budapest, Hungary"}</p>
                        <ul>
                            <li>{"Analytical Quality Assurance for a financial pricing analytics library."}</li>
                            <li>{"Engage with developer and research about new features and bugs."}</li>
                        </ul>
                    </div>
                </>
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
                <>
                    <b><a href="https://github.com/gabrielhsc95/tictactoe" target="_blank">{"tictactoe"}</a></b>
                    <p>{"More than just a Tic Tac Toe, it is binary game engine and Artificial Intelligence."}</p>
                </>
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
                <p>{"list, ordered by most likes, of blog posts."}</p>
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
                <>
                    <b><a href="https://github.com/Giovani-Costa/project_xmercury" target="_blank">{"project_xmercury"}</a></b>
                    <p>{"Manage a Tabletop RPG game using a discord bot and streamlit app."}</p>
                </>
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
                <>
                    <b><a href="https://github.com/Giovani-Costa/project_xlunar" target="_blank">{"project_xlunar"}</a></b>
                    <p>{"Discord bot to help student for standards exams."}</p>
                </>
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
