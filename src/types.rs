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
