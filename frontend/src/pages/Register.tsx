import { useState } from "react";
import { postJson, type MessageResponse } from "../api";

type Props = {
  onGoLogin: () => void;
};

// 新規登録画面
export default function Register({ onGoLogin }: Props) {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setMessage(null);
    setSubmitting(true);
    try {
      const res = await postJson<MessageResponse>("/register", {
        name,
        email,
        password,
      });
      setMessage(`${res.message} ログイン画面からログインしてください。`);
    } catch (err) {
      setError(err instanceof Error ? err.message : "登録に失敗しました");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div>
      <h2>新規登録</h2>
      <form onSubmit={handleSubmit}>
        <label>
          名前
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
          />
        </label>
        <label>
          メールアドレス
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
          />
        </label>
        <label>
          パスワード
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
        </label>
        <button type="submit" disabled={submitting}>
          {submitting ? "送信中..." : "登録"}
        </button>
      </form>

      {message && <p className="msg">{message}</p>}
      {error && <p className="err">{error}</p>}

      <p>
        すでにアカウントをお持ちの方は{" "}
        <button className="link-button" onClick={onGoLogin}>
          ログイン
        </button>
      </p>
    </div>
  );
}
