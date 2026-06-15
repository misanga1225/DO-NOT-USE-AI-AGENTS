import { useState } from "react";
import "./App.css";
import Login from "./pages/Login";
import Register from "./pages/Register";
import Dashboard from "./pages/Dashboard";

// 未ログイン時に表示
type View = "login" | "register";

function App() {
  const [view, setView] = useState<View>("login");
  // ログイン中のメールアドレス
  const [email, setEmail] = useState<string | null>(null);
  if (email) {
    return <Dashboard email={email} onLogout={() => setEmail(null)} />;
  }

  return (
    <div className="auth">
      <h1>アニメレコメンド</h1>
      {view === "login" ? (
        <Login
          onLoggedIn={(e) => setEmail(e)}
          onGoRegister={() => setView("register")}
        />
      ) : (
        <Register onGoLogin={() => setView("login")} />
      )}
    </div>
  );
}

export default App;
