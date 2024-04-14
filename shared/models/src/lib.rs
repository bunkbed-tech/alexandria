use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Resource {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub year_published: Option<i32>,
    pub thumbnail: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Tagging {
    pub id: i64,
    pub tag_id: i64,
    pub resource_id: i64,
}
