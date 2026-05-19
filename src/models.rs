use sqlx::types::chrono::{DateTime, NaiveDate, Utc};

// enumで種類を追加
// MediaType
pub enum MediaType {
    Novel,
    Anime,
    Manga,
    Game,
}

//status
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

// 作品データの構造体
// 手動で作品を追加できる機能
pub struct Work {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub description: String,
    pub episodes: i32,
    pub media_type: String,
    pub genre: String,
    pub status: String,
    pub added_at: NaiveDate,
    pub created_at: DateTime<Utc>,
}
