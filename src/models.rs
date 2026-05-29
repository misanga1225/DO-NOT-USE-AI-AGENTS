use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

// MediaType
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum MediaType {
    Novel,
    Anime,
    Manga,
    Game,
}

//status
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

// 作品データの構造体
// 手動で作品を追加できる機能
pub struct Work {
    pub title: String,
    pub author: String,
    pub description: String,
    pub episodes: Option<i32>,
    pub media_type: MediaType,
    pub genres: Vec<String>,
}

// enum変換の際のエラーの定義
#[derive(Debug, PartialEq)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "無効な文字列です")
    }
}
impl std::error::Error for ParseError {}

impl FromStr for MediaType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Novel" => Ok(MediaType::Novel),
            "Anime" => Ok(MediaType::Anime),
            "Manga" => Ok(MediaType::Manga),
            "Game" => Ok(MediaType::Game),
            _ => Err(ParseError),
        }
    }
}

impl FromStr for Status {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NotStarted" => Ok(Status::NotStarted),
            "InProgress" => Ok(Status::InProgress),
            "Completed" => Ok(Status::Completed),
            _ => Err(ParseError),
        }
    }
}

// enum -> 文字列への変換（DBのVARCHAR列に保存するときに使う）
impl MediaType {
    pub fn as_str(&self) -> &str {
        match self {
            MediaType::Novel => "Novel",
            MediaType::Anime => "Anime",
            MediaType::Manga => "Manga",
            MediaType::Game => "Game",
        }
    }
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::NotStarted => "NotStarted",
            Status::InProgress => "InProgress",
            Status::Completed => "Completed",
        }
    }
}

// 以下ユニットテスト
#[cfg(test)]
mod tests {
    //enumとFromStrをテストに取り込む
    use super::*;

    #[test]
    fn test_mediatype_from_str() {
        assert_eq!(MediaType::from_str("Novel"), Ok(MediaType::Novel));
        assert_eq!(MediaType::from_str("Game"), Ok(MediaType::Game));
        assert_eq!(MediaType::from_str("InvalidStr"), Err(ParseError));
    }

    #[test]
    fn test_status_from_str() {
        assert_eq!(Status::from_str("NotStarted"), Ok(Status::NotStarted));
        assert_eq!(Status::from_str("Completed"), Ok(Status::Completed));
        assert_eq!(Status::from_str("completed"), Err(ParseError));
    }

    #[test]
    fn test_mediatype_as_str() {
        assert_eq!(MediaType::Novel.as_str(), "Novel");
        assert_eq!(MediaType::Game.as_str(), "Game");
    }

    #[test]
    fn test_status_as_str() {
        assert_eq!(Status::InProgress.as_str(), "InProgress");
        assert_eq!(Status::Completed.as_str(), "Completed");
    }
}
