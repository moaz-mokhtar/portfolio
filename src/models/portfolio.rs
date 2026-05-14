use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortfolioData {
    pub profile: Vec<ProfileEntry>,
    pub skills: Vec<SkillGroup>,
    pub experience: Vec<ExperienceEntry>,
    pub showcases: Vec<ShowcaseEntry>,
    pub contact: Vec<ContactEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfileEntry {
    pub text: String,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillGroup {
    pub category: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExperienceEntry {
    pub title: String,
    pub period: String,
    pub bullets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShowcaseEntry {
    pub title: String,
    pub stack: String,
    pub bullets: Vec<String>,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContactEntry {
    pub platform: String,
    pub url: String,
    pub icon: String,
}

static PORTFOLIO_DATA: LazyLock<PortfolioData> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../public/data/portfolio.json"))
        .expect("portfolio.json should deserialize into PortfolioData")
});

pub fn portfolio_data() -> &'static PortfolioData {
    &PORTFOLIO_DATA
}

#[cfg(test)]
mod tests {
    use super::portfolio_data;

    #[test]
    fn parses_real_portfolio_json() {
        let data = portfolio_data();

        assert_eq!(data.profile.len(), 4);
        assert_eq!(data.skills.len(), 4);
        assert_eq!(data.experience.len(), 7);
        assert_eq!(data.showcases.len(), 5);
        assert_eq!(data.contact.len(), 4);
    }
}
