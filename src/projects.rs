use crate::{
    projects,
    window::{WindowState, WindowWidget},
};
use leptos::prelude::*;

pub enum ProjectCategory {
    Mine,
    Mentor,
    Contributor,
}

pub struct ProjectItem {
    github: String,
    description: String,
    category: ProjectCategory,
}

impl ProjectItem {
    pub fn new(github: String, description: String, category: ProjectCategory) -> Self {
        Self {
            github,
            description,
            category,
        }
    }
}

#[component]
pub fn ProjectWidget(project_item: ProjectItem) -> impl IntoView {
    let mut project_name = project_item.github.clone();
    project_name = project_name.split("/").last().unwrap().to_string();
    view! {
        <b><a href={project_item.github} target="_blank">{project_name}</a></b>
        <p>{project_item.description}</p>

    }
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let project_mine_item_1 = ProjectItem::new(
        "https://github.com/gabrielhsc95/crustacean-capital".to_string(),
        "A bond pricer in Rust.".to_string(),
        ProjectCategory::Mine,
    );
    let project_mine_1 = WindowState::new(
        String::from("project/something"),
        view! {
            <ProjectWidget project_item=project_mine_item_1 />
        },
        30,
        110,
        300,
        300,
    );
    let project_mine_item_2 = ProjectItem::new(
        "https://github.com/gabrielhsc95/gabuzando".to_string(),
        "This website! Built in Rust to showcase my experience and some of my thoughts."
            .to_string(),
        ProjectCategory::Mine,
    );
    let project_mine_2 = WindowState::new(
        String::from("project/something"),
        view! {
            <ProjectWidget project_item=project_mine_item_2 />
        },
        380,
        110,
        300,
        300,
    );
    //
    let project_mine_item_3 = ProjectItem::new(
        "https://github.com/gabrielhsc95/tictactoe".to_string(),
        "More than just a Tic Tac Toe, it is binary game engine and Artificial Intelligence."
            .to_string(),
        ProjectCategory::Mine,
    );
    // in home page
    let project_mine_3 = WindowState::new(
        String::from("project/something"),
        view! {
            <ProjectWidget project_item=project_mine_item_3 />
        },
        550,
        590,
        300,
        300,
    );
    let project_mentor_1 = ProjectItem::new(
        "https://github.com/Giovani-Costa/project_xmercury".to_string(),
        "Manage a Tabletop RPG game using a discord bot and streamlit app.".to_string(),
        ProjectCategory::Mentor,
    );
    let project_mentor_2 = ProjectItem::new(
        "https://github.com/Giovani-Costa/project_xlunar".to_string(),
        "Discord bot to help student for standards exams.".to_string(),
        ProjectCategory::Mentor,
    );
    let project_mentor = WindowState::new(
        String::from("project/mentor"),
        view! {
            <ProjectWidget project_item=project_mentor_1 />
            <br />
            <ProjectWidget project_item=project_mentor_2 />
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
