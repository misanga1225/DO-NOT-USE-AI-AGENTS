// anyhowはまだ使ってないから警告出るけど消さないで
use anyhow::Result;
use axum::Router;
use tokio;

#[tokio::main]
async fn main() {
    // ルータの作成
    // エンドポイントと，ハンドラと呼ばれるリクエストに対してどんなレスポンスを返すかの関数も後に実装
    let app = Router::new();

    // TCPリスナの作成
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {:?}", listener.local_addr().unwrap());

    // HTTPサーバの起動
    axum::serve(listener, app).await.unwrap();
}
