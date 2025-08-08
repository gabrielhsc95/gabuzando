use crate::cv::ExperienceItem;
use crate::projects::{ProjectCategory, ProjectItem};
use crate::utils::join_by_br;
use crate::window::{WindowState, WindowWidget};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let experience_item_1 = ExperienceItem::new(
        "MSCI".to_string(),
        "Aug. 2024 – Present".to_string(),
        "Quantitative Researcher (Senior Associate)".to_string(),
        "Norman, Oklahoma, United States".to_string(),
        vec![
            "Developed an AI financial analyst agent, focusing on named entity recognition and validation.".to_string(),
            "Participated in research discussions to enhance agent capabilities using various LLM models.".to_string(),
            "Understood client engagement to improve on pain points.".to_string(),
        ],
    ).to_string();
    let experience_item_2 = ExperienceItem::new(
        "MSCI".to_string(),
        "Jan. 2024 - Jul. 2024".to_string(),
        "Financial Engineer (Senior Associate)".to_string(),
        "Norman, Oklahoma, United States".to_string(),
        vec![
            "Teach Led in Testing, empowering 4 people to implement new features to an automated testing software".to_string(),
            "Managed the testing side implementation of two new pricing models.".to_string(),
        ],
    ).to_string();
    let experience_item_3 = ExperienceItem::new(
        "MSCI".to_string(),
        "Jan. 2023 - Dec. 2023".to_string(),
        "Financial Engineer (Associate)".to_string(),
        "Norman, Oklahoma, United States".to_string(),
        vec![
            "Analytical Quality Assurance for financial factor models.".to_string(),
            "Developed and reviewed financial building blocks libraries to facilitate the replication of financial models.".to_string()
        ],
    ).to_string();
    let experience_item_4 = ExperienceItem::new(
        "MSCI".to_string(),
        " Jun. 2021 - Dec. 2022".to_string(),
        "Financial Engineer (Analyst)".to_string(),
        "Norman, Oklahoma, United States and Budapest, Hungary".to_string(),
        vec![
            "Built an automated testing software for a financial pricing analytics library, and integrated it to the build pipeline.".to_string(),
            "Developed two financial models to estimate financed emissions in accordance with PCAF.".to_string(),
        ],
    ).to_string();
    let experience_item_5 = ExperienceItem::new(
        "MSCI".to_string(),
        "Sep. 2020 - Jun. 2021".to_string(),
        "Financial Engineer (Intern)".to_string(),
        "Budapest, Hungary".to_string(),
        vec![
            "Analytical Quality Assurance for a financial pricing analytics library.".to_string(),
            "Engage with developer and research about new features and bugs.".to_string(),
        ],
    )
    .to_string();
    let cv = WindowState::new(
        String::from("cv/experience"),
        join_by_br(vec![
            &experience_item_1,
            &experience_item_2,
            &experience_item_3,
            &experience_item_4,
            &experience_item_5,
        ]),
        30,
        110,
        490,
        780,
    );
    let about = WindowState::new(
        String::from("about/me"),
        String::from(
            "<p>I am a software developer, a loving partner, a proud step-dad, and an unapologetic nerd with a dash of delightful weirdness, all fueled by my Brazilian roots. Hailing from  <a href=\"https://maps.app.goo.gl/7jXanpdULSnsMbwj7\">Londrina, Paraná, Brazil</a>, my academic background is in the cosmos. I hold a Masters in Cosmology and Astrophysics.</p>
            <br />
            <p>Life took a fascinating turn, leading me to the finance industry. I have navigated roles from Financial Engineer (Analytical Quality Assurance) to my current position as a Quantitative Researcher. Ultimately, I see myself as a tool builder, constantly creating and finding solutions.</p>
            <br />
            <p>Beyond the world of finance, I am passionate about learning new things. You will often find me at the movies, happily coding personal projects, or immersed in the satisfying click of LEGO bricks.</p>"
        ),
        550,
        110,
        620,
        450,
    );
    let project_mine_item = ProjectItem::new(
        "https://github.com/gabrielhsc95/tictactoe".to_string(),
        "More than just a Tic Tac Toe, it is binary game engine and Artificial Intelligence."
            .to_string(),
        ProjectCategory::Mine,
    )
    .to_string();
    // in home page
    let project_mine = WindowState::new(
        String::from("project/something"),
        project_mine_item,
        // },
        550,
        590,
        300,
        300,
    );
    let blog = WindowState::new(
        String::from("blog/best"),
        String::from("<p>list, ordered by most likes, of blog posts.</p>"),
        870,
        590,
        300,
        450,
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
    let home = WindowState::new(
        String::from("home/greetings"),
        String::from("<p>hi</p>"),
        870,
        1070,
        300,
        100,
    );
    view! {
        <WindowWidget state=cv />
        <WindowWidget state=about />
        <WindowWidget state=project_mine />
        <WindowWidget state=blog />
        <WindowWidget state=project_mentor />
        <WindowWidget state=home />
    }
}
