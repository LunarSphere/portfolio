use serde::Deserialize;

const PROJECTS_JSON: &str = include_str!("../../data/projects.json");
const SOCIALS_JSON: &str = include_str!("../../data/socials.json");

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub title: String,
    pub slug: String,
    pub blurb: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub featured: bool,
}

impl Project {
    pub fn primary_url(&self) -> Option<&str> {
        self.live_url
            .as_deref()
            .or(self.github_url.as_deref())
            .filter(|url| !url.trim().is_empty())
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SocialLink {
    pub label: String,
    pub handle: Option<String>,
    pub url: String,
    pub blurb: String,
}

impl SocialLink {
    pub fn primary_url(&self) -> Option<&str> {
        (!self.url.trim().is_empty()).then_some(self.url.as_str())
    }
}

pub fn load_projects() -> Vec<Project> {
    serde_json::from_str(PROJECTS_JSON).expect("data/projects.json must be valid project data")
}

pub fn load_socials() -> Vec<SocialLink> {
    serde_json::from_str(SOCIALS_JSON).expect("data/socials.json must be valid social data")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_data_loads() {
        let projects = load_projects();
        assert!(!projects.is_empty());
        assert!(projects.iter().all(|project| !project.title.is_empty()));
    }

    #[test]
    fn social_data_loads() {
        let socials = load_socials();
        assert!(!socials.is_empty());
        assert!(socials.iter().all(|social| social.primary_url().is_some()));
    }
}
