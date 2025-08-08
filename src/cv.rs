use std::vec;

use crate::utils::join_by_br;
use crate::window::{WindowState, WindowWidget};
use leptos::prelude::*;

pub struct ExperienceItem {
    company: String,
    period: String,
    position: String,
    location: String,
    description: Vec<String>,
}

impl ExperienceItem {
    pub fn new(
        company: String,
        period: String,
        position: String,
        location: String,
        description: Vec<String>,
    ) -> Self {
        Self {
            company,
            period,
            position,
            location,
            description,
        }
    }

    pub fn to_string(self) -> String {
        let description = self
            .description
            .iter()
            .map(|s| format!("<li>{}</li>", s))
            .collect::<String>();
        format!("
            <div style=\"display: flex; justify-content: space-between; align-items: center; width: 100%;\">
                <div><b>{}</b></div>
                <div>{}</div>
            </div>
            <p><i>{}</i></p>
            <p>{}</p>
            <ul>
            {}
            </ul>
        ",
        self.position,
        self.period,
        self.company,
        self.location,
        description
        )
    }
}

pub struct EducationItem {
    degree: String,
    school: String,
    location: String,
    period: String,
}

impl EducationItem {
    pub fn new(degree: String, school: String, location: String, period: String) -> Self {
        Self {
            degree,
            school,
            location,
            period,
        }
    }

    pub fn to_string(self) -> String {
        format!("
            <div style=\"display: flex; justify-content: space-between; align-items: center; width: 100%;\">
                <div><b>{}</b></div>
                <div>{}</div>
            </div>
            <p><i>{}</i></p>
            <p>{}</p>
        ",
        self.degree,
        self.period,
        self.school,
        self.location,
        )
    }
}

pub struct SkillItem {
    category: String,
    skills: Vec<String>,
}

impl SkillItem {
    pub fn new(category: String, skills: Vec<String>) -> Self {
        Self { category, skills }
    }

    pub fn to_string(self) -> String {
        let joined_skills = self.skills.join(", ");
        format!(
            "
            <b>{}</b>: {}
        ",
            self.category, joined_skills
        )
    }
}

pub struct AdditionalInformationItem {
    category: String,
    description: Vec<String>,
}

impl AdditionalInformationItem {
    pub fn new(category: String, description: Vec<String>) -> Self {
        Self {
            category,
            description,
        }
    }

    pub fn to_string(self) -> String {
        let description = self
            .description
            .iter()
            .map(|s| format!("<p>{}</p>", s))
            .collect::<String>();
        format!(
            "
            <b>{}</b>
            {}
        ",
            self.category, description
        )
    }
}

#[component]
pub fn CVPage() -> impl IntoView {
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
    let experience = WindowState::new(
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
    let education_item_1 = EducationItem::new(
        "Physics (MSc)".to_string(),
        "Eötvös Loránd Tudományegyetem (ELTE)".to_string(),
        "Budapest, Hungary".to_string(),
        "Sep. 2018 – Jun. 2020".to_string(),
    )
    .to_string();
    let education_item_2 = EducationItem::new(
        "Physics (BSc)".to_string(),
        "Universidade Estadual de Londrina (UEL)".to_string(),
        "Londrina, Paraná, Brazil".to_string(),
        "Feb. 2012 – Mar. 2017".to_string(),
    )
    .to_string();
    let education_item_3 = EducationItem::new(
        "Physics (BSc)".to_string(),
        "Xavier University (XU)".to_string(),
        "Cincinnati, Ohio, United States".to_string(),
        "Aug. 2014 – May. 2015".to_string(),
    )
    .to_string();
    let education_item_4 = EducationItem::new(
        "Electromechanics (Technical Course)".to_string(),
        "Serviço Nacional de Aprendizagem Industrial (SENAI)".to_string(),
        "Londrina, Paraná, Brazil".to_string(),
        "Feb. 2010 – Dec. 2011".to_string(),
    )
    .to_string();
    let education = WindowState::new(
        String::from("cv/education"),
        join_by_br(vec![
            &education_item_1,
            &education_item_2,
            &education_item_3,
            &education_item_4,
        ]),
        550,
        110,
        620,
        500,
    );
    let skill_item_1 = SkillItem::new(
        "Languages".to_string(),
        vec![
            "Native Portuguese".to_string(),
            "Advanced English".to_string(),
            "Intermediate Spanish".to_string(),
        ],
    )
    .to_string();
    let skill_item_2 = SkillItem::new(
        "Programming languages".to_string(),
        vec![
            "Rust (leptos, clap, derive_more, serde)".to_string(),
            "Python (pandas, numpy, Django, FastAPI, LangChain, LangGraph, Pydantic)".to_string(),
            "Matlab".to_string(),
            "Mathematica".to_string(),
            "C".to_string(),
            "C++".to_string(),
            "Java".to_string(),
        ],
    )
    .to_string();
    let skill_item_3 = SkillItem::new(
        "Tools".to_string(),
        vec![
            "Microsoft Office".to_string(),
            "Image Editing".to_string(),
            "Audio Editing".to_string(),
            "Video Editing".to_string(),
        ],
    )
    .to_string();
    let skill_item_4 = SkillItem::new(
        "Solf".to_string(),
        vec![
            "Public Speaking".to_string(),
            "Teamwork".to_string(),
            "Leadership".to_string(),
            "Problem-Solving".to_string(),
        ],
    )
    .to_string();
    let skills = WindowState::new(
        String::from("cv/skills"),
        join_by_br(vec![
            &skill_item_1,
            &skill_item_2,
            &skill_item_3,
            &skill_item_4,
        ]),
        550,
        630,
        620,
        260,
    );
    let additional_information_item_1 = AdditionalInformationItem::new(
        "Volunteering".to_string(),
        vec![
            "Teaching programming for a countryside kid since 2022. ".to_string(),
            "Taught Mathematics to countryside kids in Brazil during the COVID-19 pandemic (2021)."
                .to_string(),
            "stuff".to_string(),
        ],
    )
    .to_string();

    let additional_information_item_2 = AdditionalInformationItem::new(
        "Awards".to_string(),
        vec![
            "Best Student in the Electromechanics Program, 2011".to_string(),
            "Bronze Medal in the Brazilian Olympics of Astronomy and Astronautics, 2009"
                .to_string(),
            "Honorable Mention in the Brazilian Olympics of Mathematics for Public Schools, 2007"
                .to_string(),
        ],
    )
    .to_string();

    let additional_information_item_3 = AdditionalInformationItem::new(
        "Seminars".to_string(),
        vec![
            "Presented Introduction to Cosmic Topology, Londrina-PR, Brazil, 2016".to_string(),
            "Presented Physics of Boomerang, Londrina-PR, Brazil, 2013".to_string(),
            "Attended IV Jayme Tiomno School of Cosmology at National Observatory, Rio de Janeiro-RJ, Brazil, 2016".to_string(),
            "Attended Physics Week at the State University of Londrina XVI (2011), XVII (2012), XVIII (2013), XX (2015), and XXI (2016)".to_string(),
        ],
    ).to_string();
    let additional_information_item_4 = AdditionalInformationItem::new(
        "Certifications".to_string(),
        vec![
            "Stipendium Hungaricum by Tempus Public Foundation between Sep. 2018 – Jul. 2020".to_string(),
            "Tutorial Education Program (PET) by the Brazilian Ministry of Education (MEC) between Mar. 2013 – Mar. 2014  and Oct. 2015 – Mar. 2017".to_string(),
            "Science Without Borders by Coordination for the Improvement of Higher Education Personnel (CAPES) and National Council for Scientific and Technological Development (CNPq) between Aug. 2014 – May 2015".to_string(),
            "FIEP Scholarship by Federation of Industries of Paraná State (FIEP) between  Feb. 2010 – Dec. 2011".to_string(),
        ],
    ).to_string();

    let additional_information = WindowState::new(
        String::from("cv/additional_information"),
        join_by_br(vec![
            &additional_information_item_1,
            &additional_information_item_2,
            &additional_information_item_3,
            &additional_information_item_4,
        ]),
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
