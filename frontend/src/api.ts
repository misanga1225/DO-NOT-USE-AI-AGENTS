// バックエンドへの通信をまとめたファイル

const BASE = "/api";

export type MessageResponse = { message: string };

// 共通のレスポンス処理
async function handle<T>(res: Response): Promise<T> {
  const data = await res.json();
  if (!res.ok) {
    const message =
      data && typeof data.message === "string"
        ? data.message
        : `エラー: ${res.status}`;
    throw new Error(message);
  }
  return data as T;
}

// GETリクエスト
export async function getJson<T>(path: string): Promise<T> {
  const res = await fetch(`${BASE}${path}`);
  return handle<T>(res);
}

// POSTリクエスト
export async function postJson<T>(path: string, body: unknown): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  return handle<T>(res);
}
