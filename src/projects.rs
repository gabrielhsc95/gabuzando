use crate::window::{WindowState, WindowWidget};
use leptos::prelude::*;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let project_mine_1 = WindowState::new(
        String::from("project/something"),
        view! {<p>"a project I did"</p>},
        30,
        110,
        300,
        300,
    );
    let project_mine_2 = WindowState::new(
        String::from("project/something"),
        view! {<p>"a project I did"</p>},
        380,
        110,
        300,
        300,
    );
    // in home page
    let project_mine_3 = WindowState::new(
        String::from("project/something"),
        view! {<p>"a project I did"</p>},
        550,
        590,
        300,
        300,
    );
    let project_mentor = WindowState::new(
        String::from("project/mentor"),
        view! {
            <p>"a project a mentored"</p>
        },
        30,
        920,
        820,
        250,
    );
    view! {
        <WindowWidget state=project_mine_1 />
        <WindowWidget state=project_mine_2 />
        <WindowWidget state=project_mine_3 />
        <WindowWidget state=project_mentor />
    }
}
