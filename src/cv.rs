use crate::window::{WindowState, WindowWidget};
use leptos::prelude::*;

#[component]
pub fn CVPage() -> impl IntoView {
    let experience = WindowState::new(
        String::from("cv/experience"),
        view! {
            <p>"professional experience"</p>
        },
        30,
        110,
        490,
        780,
    );
    let education = WindowState::new(
        String::from("cv/education"),
        view! {
            <p>"education"</p>
        },
        550,
        110,
        620,
        500,
    );
    let skills = WindowState::new(
        String::from("cv/skills"),
        view! {
            <p>"skills"</p>
        },
        550,
        630,
        620,
        260,
    );
    let additional_information = WindowState::new(
        String::from("cv/additional_information"),
        view! {
            <p>"stuff"</p>
        },
        30,
        920,
        1140,
        260,
    );
    view! {
        <WindowWidget state=experience />
        <WindowWidget state=education />
        <WindowWidget state=skills />
        <WindowWidget state=additional_information />
    }
}
