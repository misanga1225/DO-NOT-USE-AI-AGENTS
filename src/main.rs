//! 実行ファイルのエントリポイント。ライブラリの run() を呼び出すだけ。
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    do_not_use_AI_agent::server::run().await?;
    Ok(())
}
