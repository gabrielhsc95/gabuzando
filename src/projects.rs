use crate::utils::join_by_br;
use crate::window::{WindowState, WindowWidget};
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

    pub fn to_string(self) -> String {
        let mut project_name = self.github.clone();
        project_name = project_name.split("/").last().unwrap().to_string();
        format!(
            "
            <b><a href=\"{}\" target=\"_blank\">{}</a></b>
            <p>{}</p>
        ",
            self.github, project_name, self.description,
        )
    }
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let project_mine_item_1 = ProjectItem::new(
        "https://github.com/gabrielhsc95/crustacean-capital".to_string(),
        "A bond pricer in Rust.".to_string(),
        ProjectCategory::Mine,
    )
    .to_string();
    let project_mine_1 = WindowState::new(
        String::from("project/something"),
        project_mine_item_1,
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
    )
    .to_string();
    let project_mine_2 = WindowState::new(
        String::from("project/something"),
        project_mine_item_2,
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
    )
    .to_string();
    // in home page
    let project_mine_3 = WindowState::new(
        String::from("project/something"),
        project_mine_item_3,
        550,
        590,
        300,
        300,
    );
    let project_mentor_1 = ProjectItem::new(
        "https://github.com/Giovani-Costa/project_xmercury".to_string(),
        "Manage a Tabletop RPG game using a discord bot and streamlit app.".to_string(),
        ProjectCategory::Mentor,
    )
    .to_string();
    let project_mentor_2 = ProjectItem::new(
        "https://github.com/Giovani-Costa/project_xlunar".to_string(),
        "Discord bot to help student for standards exams.".to_string(),
        ProjectCategory::Mentor,
    )
    .to_string();
    let project_mentor = WindowState::new(
        String::from("project/mentor"),
        join_by_br(vec![&project_mentor_1, &project_mentor_2]),
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
