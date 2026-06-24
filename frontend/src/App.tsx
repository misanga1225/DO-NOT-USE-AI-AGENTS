import { useState } from "react";
import "./App.css";
import Login from "./pages/Login";
import Register from "./pages/Register";
import Dashboard from "./pages/Dashboard";

// 未ログイン時に表示
type View = "login" | "register";

// 背景に漂う粒子（ALTER EGO 的な静けさの演出）
const PARTICLES = Array.from({ length: 16 });

function Backdrop() {
  return (
    <div className="particles" aria-hidden="true">
      {PARTICLES.map((_, i) => (
        <span
          key={i}
          style={{
            left: `${(i * 6.3 + 3) % 100}%`,
            animationDuration: `${12 + (i % 5) * 4}s`,
            animationDelay: `${(i % 7) * 1.7}s`,
          }}
        />
      ))}
    </div>
  );
}

function App() {
  const [view, setView] = useState<View>("login");
  // ログイン中のメールアドレス
  const [email, setEmail] = useState<string | null>(null);

  if (email) {
    return (
      <>
        <Backdrop />
        <Dashboard email={email} onLogout={() => setEmail(null)} />
      </>
    );
  }

  return (
    <>
      <Backdrop />
      <div className="auth">
        <div className="brand">
          <p className="hud-tag">Sibyl System // Psycho-Hazard Archive</p>
          <h1 className="title-line">アニメレコメンド</h1>
          <p className="brand-sub">傾向を測定し、次に対峙すべき一作を診断する。</p>
        </div>
        {view === "login" ? (
          <Login
            onLoggedIn={(e) => setEmail(e)}
            onGoRegister={() => setView("register")}
          />
        ) : (
          <Register onGoLogin={() => setView("login")} />
        )}
      </div>
    </>
  );
}

export default App;
