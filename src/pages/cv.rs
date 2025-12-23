use yew::prelude::*;
use crate::components::window::WindowProps;
use crate::components::loading::Loading;
use crate::hooks::{use_fetch, FetchState};
use crate::types::{ExperienceContent, EducationContent, SkillsContent, AdditionalInfoContent};
use crate::components::language_context::use_language;

#[function_component(ExperienceLoader)]
pub fn experience_loader() -> Html {
    let fetch_state = use_fetch::<ExperienceContent>("/text/cv/experience.json");
    let language = use_language().language;

    match fetch_state {
        FetchState::Success(data) => html! {
            <>
                { for data.jobs.iter().map(|job| html! {
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{ job.title.get(language) }</b></div>
                            <div>{&job.dates}</div>
                        </div>
                        <p><i>{&job.company}</i></p>
                        <p>{ job.location.get(language) }</p>
                        <ul>
                            { for job.items.iter().map(|item| html! { <li>{ item.get(language) }</li> }) }
                        </ul>
                    </div>
                }) }
            </>
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

#[function_component(EducationLoader)]
fn education_loader() -> Html {
    let fetch_state = use_fetch::<EducationContent>("/text/cv/education.json");
    let language = use_language().language;

    match fetch_state {
        FetchState::Success(data) => html! {
            <>
                { for data.items.iter().map(|item| html! {
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{ item.degree.get(language) }</b></div>
                            <div>{&item.dates}</div>
                        </div>
                        <p><i>{&item.institution}</i></p>
                        <p>{ item.location.get(language) }</p>
                    </div>
                }) }
            </>
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

#[function_component(SkillsLoader)]
fn skills_loader() -> Html {
    let fetch_state = use_fetch::<SkillsContent>("/text/cv/skills.json");
    let language = use_language().language;

    match fetch_state {
        FetchState::Success(data) => html! {
            <>
                { for data.skills.iter().map(|skill| html! {
                    <p><b>{&skill.category}</b>{ skill.text.get(language) }</p>
                }) }
            </>
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

#[function_component(AdditionalInfoLoader)]
fn additional_info_loader() -> Html {
    let fetch_state = use_fetch::<AdditionalInfoContent>("/text/cv/additional_info.json");
    let language = use_language().language;

    match fetch_state {
        FetchState::Success(data) => html! {
            <>
                { for data.sections.iter().map(|section| html! {
                    <div>
                        <b>{ section.title.get(language) }</b>
                        { for section.items.iter().map(|item| html! { <p>{ item.get(language) }</p> }) }
                    </div>
                }) }
            </>
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

pub fn get_cv_windows() -> Vec<WindowProps> {
    vec![
        // Experience (Cells 1, 4 - Left Column)
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
        // Education (Cells 2, 3 - Top Row, Center+Right)
        WindowProps {
            title: AttrValue::from("cv/education"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <EducationLoader />
            }]),
            x: 34.6,
            y: 10.0,
            width: 64.2,
            height: 27.3,
            buffer: 20.0,
        },
        // Skills (Cells 5, 6 - Middle Row, Center+Right)
        WindowProps {
            title: AttrValue::from("cv/skills"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <SkillsLoader />
            }]),
            x: 34.6,
            y: 39.3,
            width: 64.2,
            height: 26.3,
            buffer: 20.0,
        },
        // Additional Information (Cells 7, 8, 9 - Bottom Row)
        WindowProps {
            title: AttrValue::from("cv/additional_information"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <AdditionalInfoLoader />
            }]),
            x: 1.0,
            y: 67.6,
            width: 97.8,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
