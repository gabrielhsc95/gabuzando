use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct SimpleTextContent {
    pub text: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct GreetingsList {
    pub greetings: Vec<String>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct CountriesContent {
    pub based: String,
    pub from: String,
    pub lived: String,
    pub visited: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ContactLink {
    pub icon: String,
    pub alt: String,
    pub text: String,
    pub url: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ContactContent {
    pub links: Vec<ContactLink>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct QuotesContent {
    pub quotes: Vec<Quote>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct JobItem {
    pub title: String,
    pub dates: String,
    pub company: String,
    pub location: String,
    pub items: Vec<String>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ExperienceContent {
    pub jobs: Vec<JobItem>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct EducationItem {
    pub degree: String,
    pub dates: String,
    pub institution: String,
    pub location: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct EducationContent {
    pub items: Vec<EducationItem>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct SkillItem {
    pub category: String,
    pub text: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct SkillsContent {
    pub skills: Vec<SkillItem>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct InfoSection {
    pub title: String,
    pub items: Vec<String>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct AdditionalInfoContent {
    pub sections: Vec<InfoSection>,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub enum ProjectRole {
    Mine,
    Mentor,
    Contributor,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ProjectItem {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub description: String,
    pub role: ProjectRole,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ProjectsContent {
    pub projects: Vec<ProjectItem>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct BlogItem {
    pub id: String,
    pub title: String,
    pub url: String,
    pub description: String,
    pub summary: String,
    pub content: String,
    pub likes: u32,
    pub date: String,
    pub category: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct BlogList {
    pub posts: Vec<BlogItem>,
}
