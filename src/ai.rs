use crate::error::AppError;
use reqwest::Client;
use serde_json::json;
use std::env::var;

// 内部レコメンド機能
pub async fn recommend_from_list (completed: Vec<String>,not_started: Vec<String>) -> Result<String, AppError> {

    let completed_works = completed.join("、");
    let not_started_works = not_started.join("、");
    
    let prompt =format!(
        "以下はユーザーが視聴・読了済みの作品です：
        {completed_works}     

        以下はまだ手をつけていない作品です：
        {not_started_works}

        視聴済み作品から好みを読み取り、未視聴リストの中から次に楽しむべき作品を1つ選んでください。
        作品名と理由を簡潔に教えてください。"
        );

    // APIキーを環境変数から取り込む
    let api_key = var("ANTHROPIC_API_KEY").map_err(|_| AppError::External("ANTHROPIC_API_KEYが設定されていません".to_string()))?;

    // リクエストボディをJsonにして定義
    let body = json!({
        "model": "claude-sonnet-4-6",
        "max_tokens": 1024,
        "messages": [{"role": "user", "content": prompt}]// TODO: とりあえず1回片道想定．後ほど複数回できるようにするかも．
    });

    // POSTリクエストを送る
    let client = Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::External(e.to_string()))?;

    // レスポンスのJsonからテキストを取り出す
    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::External(e.to_string()))?;

    let text = json["content"][0]["text"]
        .as_str()
        .ok_or(AppError::External("レスポンスの解析に失敗しました".to_string()))?
        .to_string();
    Ok(text)

}

// 外部レコメンド機能
pub async fn recommend_new (){

}