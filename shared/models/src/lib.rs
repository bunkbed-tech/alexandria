use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Resource {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub year_published: Option<i32>,
    pub thumbnail: String,
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
