use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct BoardGame {
    pub id: Option<i32>,
    pub meta: ResourceMeta,
}

impl Resource for BoardGame {
    fn meta(&self) -> &ResourceMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut ResourceMeta {
        &mut self.meta
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct VideoGame {
    pub id: Option<i32>,
    pub meta: ResourceMeta,
}

impl Resource for VideoGame {
    fn meta(&self) -> &ResourceMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut ResourceMeta {
        &mut self.meta
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Manga {
    pub id: Option<i32>,
    pub meta: ResourceMeta,
}
impl Resource for Manga {
    fn meta(&self) -> &ResourceMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut ResourceMeta {
        &mut self.meta
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct AnimeTVShow {
    pub id: Option<i32>,
    pub meta: ResourceMeta,
}
impl Resource for AnimeTVShow {
    fn meta(&self) -> &ResourceMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut ResourceMeta {
        &mut self.meta
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct AnimeMovie {
    pub id: Option<i32>,
    pub meta: ResourceMeta,
}
impl Resource for AnimeMovie {
    fn meta(&self) -> &ResourceMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut ResourceMeta {
        &mut self.meta
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct VisualNovel {
    pub id: Option<i32>,
    pub meta: ResourceMeta,
}
impl Resource for VisualNovel {
    fn meta(&self) -> &ResourceMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut ResourceMeta {
        &mut self.meta
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct LightNovel {
    pub id: Option<i32>,
    pub meta: ResourceMeta,
}
impl Resource for LightNovel {
    fn meta(&self) -> &ResourceMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut ResourceMeta {
        &mut self.meta
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct ResourceMeta {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub year_published: Option<i32>,
    pub thumbnail: Option<String>,
    pub api_id: i32,
}

pub trait Resource {
    fn meta(&self) -> &ResourceMeta;
    fn meta_mut(&mut self) -> &mut ResourceMeta;

    fn title(&self) -> &String {
        &self.meta().title
    }

    fn id(&self) -> Option<i32> {
        self.meta().id
    }

    fn description(&self) -> &Option<String> {
        &self.meta().description
    }

    fn year_published(&self) -> Option<i32> {
        self.meta().year_published
    }

    fn thumbnail(&self) -> &Option<String> {
        &self.meta().thumbnail
    }

    fn api_id(&self) -> i32 {
        self.meta().api_id
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AlexandriaResource {
    AnimeMovie(AnimeMovie),
    AnimeTVShow(AnimeTVShow),
    Manga(Manga),
    LightNovel(LightNovel),
    VideoGame(VideoGame),
    BoardGame(BoardGame),
    VisualNovel(VisualNovel),
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
    pub resources: Vec<AlexandriaResource>,
    pub errors: Vec<String>,
}

impl Display for ResourceMeta {
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
