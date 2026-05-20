use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
// 今日はパッケージ，クレート，モジュールについて勉強しました！実装はごめんちょっと間に合わなかった！
// 明日中に追加したくなったら連絡するかもしれない！ 2026/05/20.21

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
