use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

// FIXME define models for different APIs to avoid api_id duplication

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Resource {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub year_published: Option<i32>,
    pub thumbnail: Option<String>,
    pub api_id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Tagging {
    pub id: i32,
    pub tag_id: i32,
    pub resource_id: i32,
}

#[derive(Default, Deserialize, Serialize)]
pub struct SearchResults {
    pub resources: Vec<Resource>,
    pub errors: Vec<String>,
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} ({})",
            self.title,
            self.id
                .map_or(String::from("untracked"), |id| id.to_string())
        )
    }
}
