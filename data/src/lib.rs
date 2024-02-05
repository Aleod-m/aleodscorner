use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct ArticleSummary {
    pub title: String,
    pub summary: String,
    pub id: u32,
}
