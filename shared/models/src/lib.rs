use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Resource {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub year_published: i64,
    pub owned: bool,
    pub want_to_own: bool,
    pub want_to_try: bool,
}
