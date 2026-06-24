import { useState } from "react";
import { getJson, postJson, type MessageResponse } from "../api";

type Props = {
  email: string;
  onLogout: () => void;
};

// バックエンドのenumに対応
const MEDIA_TYPES = ["Anime", "Manga", "Novel", "Game"] as const;
const STATUSES = ["NotStarted", "InProgress", "Completed"] as const;
const STRATEGIES = [
  { value: "random", label: "ランダム" },
  { value: "ai", label: "AI（一覧から）" },
  { value: "newai", label: "AI（新作）" },
] as const;

type MediaType = (typeof MEDIA_TYPES)[number];
type StatusType = (typeof STATUSES)[number];
type StrategyType = (typeof STRATEGIES)[number]["value"];

// ログイン後画面
export default function Dashboard({ email, onLogout }: Props) {
  const [userId, setUserId] = useState(1);

  const [titles, setTitles] = useState<string[]>([]);
  const [listError, setListError] = useState<string | null>(null);

  const [strategy, setStrategy] = useState<StrategyType>("random");
  const [recommend, setRecommend] = useState<string | null>(null);
  const [recError, setRecError] = useState<string | null>(null);

  const [title, setTitle] = useState("");
  const [author, setAuthor] = useState("");
  const [description, setDescription] = useState("");
  const [episodes, setEpisodes] = useState("");
  const [mediaType, setMediaType] = useState<MediaType>("Anime");
  const [status, setStatus] = useState<StatusType>("NotStarted");
  const [genres, setGenres] = useState("");
  const [workMsg, setWorkMsg] = useState<string | null>(null);
  const [workErr, setWorkErr] = useState<string | null>(null);

  // 一覧取得
  const loadList = async () => {
    setListError(null);
    try {
      const list = await getJson<string[]>(`/list?user_id=${userId}`);
      setTitles(list.filter((t) => t.trim() !== ""));
    } catch (e) {
      setListError(e instanceof Error ? e.message : "取得に失敗しました");
    }
  };

  // おすすめを取得
  const getRecommend = async () => {
    setRecError(null);
    setRecommend(null);
    try {
      const res = await getJson<MessageResponse>(
        `/recommendations?user_id=${userId}&strategy=${strategy}`,
      );
      setRecommend(res.message);
    } catch (e) {
      setRecError(e instanceof Error ? e.message : "取得に失敗しました");
    }
  };

  // 作品を登録
  const addWork = async (e: React.FormEvent) => {
    e.preventDefault();
    setWorkErr(null);
    setWorkMsg(null);
    try {
      const res = await postJson<MessageResponse>("/works", {
        user_id: userId,
        title,
        author,
        description,
        episodes: episodes === "" ? null : Number(episodes),
        media_type: mediaType,
        genres: genres
          .split(",")
          .map((g) => g.trim())
          .filter((g) => g !== ""),
        status,
      });
      setWorkMsg(res.message);
      // 入力欄をクリア
      setTitle("");
      setAuthor("");
      setDescription("");
      setEpisodes("");
      setGenres("");
    } catch (err) {
      setWorkErr(err instanceof Error ? err.message : "登録に失敗しました");
    }
  };

  return (
    <div className="dashboard">
      <header className="dashboard-header">
        <div>
          <p className="hud-tag">Sibyl System // Console</p>
          <h1>アニメレコメンド</h1>
        </div>
        <button className="link-button" onClick={onLogout}>
          ログアウト
        </button>
      </header>
      <p className="welcome">
        AUTHENTICATED INSPECTOR &mdash; <b>{email}</b>
      </p>

      <label>
        ユーザID
        <input
          type="number"
          value={userId}
          onChange={(e) => setUserId(Number(e.target.value))}
        />
      </label>

      {/* 作品一覧 */}
      <section className="card">
        <p className="section-tag">// 01 Archive</p>
        <h2>作品一覧</h2>
        <button onClick={loadList}>一覧を取得</button>
        {listError && <p className="err">{listError}</p>}
        <ul>
          {titles.length === 0 ? (
            <li>（まだありません）</li>
          ) : (
            titles.map((t, i) => <li key={i}>{t}</li>)
          )}
        </ul>
      </section>

      {/* おすすめ */}
      <section className="card">
        <p className="section-tag">// 02 Sibyl</p>
        <h2>おすすめ</h2>
        <div className="toolbar">
          <select
            value={strategy}
            onChange={(e) => setStrategy(e.target.value as StrategyType)}
          >
            {STRATEGIES.map((s) => (
              <option key={s.value} value={s.value}>
                {s.label}
              </option>
            ))}
          </select>
          <button onClick={getRecommend}>おすすめを取得</button>
        </div>
        {recError && <p className="err">{recError}</p>}
        {recommend && <p className="readout">{recommend}</p>}
      </section>

      {/* 作品登録 */}
      <section className="card">
        <p className="section-tag">// 03 Enroll</p>
        <h2>作品を登録</h2>
        <form onSubmit={addWork}>
          <label>
            タイトル
            <input
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              required
            />
          </label>
          <label>
            作者
            <input
              value={author}
              onChange={(e) => setAuthor(e.target.value)}
              required
            />
          </label>
          <label>
            説明
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
          </label>
          <label>
            話数（任意）
            <input
              type="number"
              value={episodes}
              onChange={(e) => setEpisodes(e.target.value)}
            />
          </label>
          <label>
            種別
            <select
              value={mediaType}
              onChange={(e) => setMediaType(e.target.value as MediaType)}
            >
              {MEDIA_TYPES.map((m) => (
                <option key={m} value={m}>
                  {m}
                </option>
              ))}
            </select>
          </label>
          <label>
            ジャンル（カンマ区切り）
            <input
              value={genres}
              onChange={(e) => setGenres(e.target.value)}
              placeholder="例: アクション, SF"
            />
          </label>
          <label>
            ステータス
            <select
              value={status}
              onChange={(e) => setStatus(e.target.value as StatusType)}
            >
              {STATUSES.map((s) => (
                <option key={s} value={s}>
                  {s}
                </option>
              ))}
            </select>
          </label>
          <button type="submit">登録</button>
        </form>
        {workErr && <p className="err">{workErr}</p>}
        {workMsg && <p className="msg">{workMsg}</p>}
      </section>
    </div>
  );
}
