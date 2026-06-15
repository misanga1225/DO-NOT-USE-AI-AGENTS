import { useState } from "react";
import { postJson, type MessageResponse } from "../api";

type Props = {
  onLoggedIn: (email: string) => void;
  onGoRegister: () => void;
};

// ログイン画面
export default function Login({ onLoggedIn, onGoRegister }: Props) {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setSubmitting(true);
    try {
      await postJson<MessageResponse>("/login", { email, password });
      onLoggedIn(email);
    } catch (err) {
      setError(err instanceof Error ? err.message : "ログインに失敗しました");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div>
      <h2>ログイン</h2>
      <form onSubmit={handleSubmit}>
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
          {submitting ? "送信中..." : "ログイン"}
        </button>
      </form>

      {error && <p className="err">{error}</p>}

      <p>
        アカウントがない方は{" "}
        <button className="link-button" onClick={onGoRegister}>
          新規登録
        </button>
      </p>
    </div>
  );
}
