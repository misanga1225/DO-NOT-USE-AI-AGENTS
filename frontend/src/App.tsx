import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [titles, setTitles] = useState<string[]>([]); // 作品タイトルの配列
  const [loading, setLoading] = useState(true); // 読み込み中フラグ
  const [error, setError] = useState<string | null>(null); // エラー文（無ければnull）

  useEffect(() => {
    const load = async () => {
      try {
        // /api/listがVite経由で:3000/listに転送
        const res = await fetch("/api/list?user_id=1");

        if (!res.ok) {
          throw new Error(`サーバーエラー: ${res.status}`);
        }

        // JSON配列で返る
        const list: string[] = await res.json();

        // 空タイトルを除外し保存
        setTitles(list.filter((title) => title.trim() !== ""));
      } catch (e) {
        setError(e instanceof Error ? e.message : "取得に失敗しました");
      } finally {
        setLoading(false);
      }
    };

    load();
  }, []);

  // 状態に応じて表示を出し分け
  return (
    <>
      <h1>アニメレコメンド</h1>

      {loading && <p>読み込み中...</p>}

      {error && <p style={{ color: "red" }}>エラー: {error}</p>}

      {!loading && !error && (
        <ul>
          {titles.length === 0 ? (
            <li>作品がまだありません</li>
          ) : (
            titles.map((title, i) => <li key={i}>{title}</li>)
          )}
        </ul>
      )}
    </>
  );
}

export default App;
