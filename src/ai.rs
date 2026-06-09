use crate::error::AppError;
use reqwest::Client;
use serde_json::json;
use std::env::var;

// 内部レコメンド機能
pub async fn recommend_from_list(
    completed: Vec<String>,
    not_started: Vec<String>,
) -> Result<String, AppError> {
    let prompt = build_prompt(&completed, &not_started);
    let response_json = request_completion(&prompt).await?;
    let text = extract_text(&response_json)?;
    Ok(text)
}

// プロンプト組み立て
// 非同期処理でもなければ今のところai.rsで完結しているため，pubにする必要なし
fn build_prompt(completed: &[String], not_started: &[String]) -> String {
    let completed_works = completed.join("、");
    let not_started_works = not_started.join("、");

    format!(
          "以下はユーザーが視聴・読了済みの作品です：
          {completed_works}

          以下はまだ手をつけていない作品です：
          {not_started_works}

          視聴済み作品から好みを読み取り、未視聴リストの中から次に楽しむべき作品を1つ選んでください。
          作品名と理由を簡潔に教えてください。"
      )
}

// ClaudeAPIへリクエストを送る
async fn request_completion(prompt: &str) -> Result<serde_json::Value, AppError> {
    let api_key = var("ANTHROPIC_API_KEY")
        .map_err(|_| AppError::External("ANTHROPIC_API_KEYが設定されていません".to_string()))?;

    let body = json!({
        "model": "claude-sonnet-4-6",
        "max_tokens": 1024,
        "messages": [{"role": "user", "content": prompt}]
    });

    let client = Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::External(e.to_string()))?;

    // HTTPステータスコードのチェック
    let status = response.status();
    if !status.is_success() {
        let detail = response.text().await.unwrap_or_default();
        return Err(AppError::External(format!(
            "APIがエラーを返しました (status {status}): {detail}"
        )));
    }

    response
        .json()
        .await
        .map_err(|e| AppError::External(e.to_string()))
}

// レスポンスJSONから本文テキストを取り出す
fn extract_text(json: &serde_json::Value) -> Result<String, AppError> {
    let text = json["content"][0]["text"]
        .as_str()
        .ok_or(AppError::External(
            "レスポンスの解析に失敗しました".to_string(),
        ))?
        .to_string();
    Ok(text)
}

// 外部レコメンド機能
pub async fn recommend_new() {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extract_text_ok() {
        let v = json!({"content": [{ "type": "text", "text": "進撃の巨人おすすめ" }] });
        assert_eq!(extract_text(&v).unwrap(), "進撃の巨人おすすめ");
    }

    #[test]
    fn extract_text_err() {
        let v = json!({ "type": "error", "error": { "message": "invalid api key" } });
        assert!(extract_text(&v).is_err());
    }
}
