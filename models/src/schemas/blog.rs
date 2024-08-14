use serde::Serialize;
use utoipa::ToSchema;

use crate::domains::blog;

#[derive(Serialize, ToSchema)]
pub struct BlogSchema {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
}

impl From<blog::Model> for BlogSchema {
    fn from(blog: blog::Model) -> Self {
        Self {
            id: blog.id as u32,
            title: blog.title,
            content: blog.content,
            author_id: blog.author_id as u32,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct BlogListSchema {
    pub blogs: Vec<BlogSchema>,
}

impl From<Vec<blog::Model>> for BlogListSchema {
    fn from(blogs: Vec<blog::Model>) -> Self {
        Self {
            blogs: blogs.into_iter().map(BlogSchema::from).collect(),
        }
    }
}
