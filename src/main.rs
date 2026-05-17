// anyhowはまだ使ってないから警告出るけど消さないで
use anyhow::Result;
use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::net::SocketAddr;

mod db;

// 作品データの構造体
// 手動で作品を追加できる機能
struct Work {
    id: u32,
    title: String,
    author: String,
    media_type: String,
    genre: String,
    status: String,   //視聴・プレイ状況
    added_at: String, //追加日時(あとでDateTime<Utc>に変えるかも)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // 動作確認用データ そのため上に置かせてもらいました
    // 後で作品を保存するためのリストも実装
    let work1 = Work {
        id: 1,
        title: String::from("とある魔術の禁書目録"),
        author: String::from("鎌池和馬"),
        media_type: String::from("小説"),
        genre: String::from("ファンタジー"),
        status: String::from("未完了"),
        added_at: String::from("2026-06-01"),
    };

    println!("作品名： {}", work1.title);

    let pool = db::establish_connection().await;
    // ルータの作成
    // エンドポイントと，ハンドラと呼ばれるリクエストに対してどんなレスポンスを返すかの関数も後に実装
    let app = Router::new().route("/", get(root)).with_state(pool);

    // TCPリスナの作成
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {:?}", listener.local_addr().unwrap());

    // HTTPサーバの起動
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Anime Recommend API"
}
