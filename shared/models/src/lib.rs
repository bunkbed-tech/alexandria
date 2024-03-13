use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Resource {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub year_published: Option<i32>,
    pub owned: bool,
    pub want_to_own: bool,
    pub want_to_try: bool,
}
