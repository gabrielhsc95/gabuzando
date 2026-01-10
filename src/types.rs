use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct LocalizedText {
    pub en: String,
    pub pt: String,
}

impl LocalizedText {
    pub fn get(&self, lang: Language) -> &str {
        match lang {
            Language::English => &self.en,
            Language::Portuguese => {
                if self.pt.is_empty() {
                    &self.en
                } else {
                    &self.pt
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct SimpleTextContent {
    pub text: LocalizedText,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Language {
    English,
    Portuguese,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct GreetingsList {
    pub greetings: Vec<String>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct CountriesContent {
    pub based: LocalizedText,
    pub from: LocalizedText,
    pub lived: LocalizedText,
    pub visited: LocalizedText,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ContactLink {
    pub icon: String,
    pub alt: String,
    pub text: LocalizedText,
    pub url: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ContactContent {
    pub links: Vec<ContactLink>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Quote {
    pub text: LocalizedText,
    pub author: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct QuotesContent {
    pub quotes: Vec<Quote>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct JobItem {
    pub title: LocalizedText,
    pub dates: String,
    pub company: String,
    pub location: LocalizedText,
    pub items: Vec<LocalizedText>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ExperienceContent {
    pub jobs: Vec<JobItem>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct EducationItem {
    pub degree: LocalizedText,
    pub dates: String,
    pub institution: String,
    pub location: LocalizedText,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct EducationContent {
    pub items: Vec<EducationItem>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct SkillItem {
    pub category: String,
    pub text: LocalizedText,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct SkillsContent {
    pub skills: Vec<SkillItem>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct InfoSection {
    pub title: LocalizedText,
    pub items: Vec<LocalizedText>,
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
    pub description: LocalizedText,
    pub role: ProjectRole,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ProjectsContent {
    pub projects: Vec<ProjectItem>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct BlogItem {
    pub id: String,
    pub title: LocalizedText,
    pub url: String,
    pub summary: LocalizedText,
    pub content: LocalizedText,
    pub likes: u32,
    pub date: String,
    pub category: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct BlogList {
    pub posts: Vec<BlogItem>,
}
