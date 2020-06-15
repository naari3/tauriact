import React from "react";
import { open, setTitle } from "tauri/api/window";
import logo from "./logo.svg";
import "./App.css";

const App: React.FC = () => {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <p>also hello tauriact</p>
        <button
          type="button"
          onClick={(): void => {
            open("http://example.com");
            setTitle("asd");
          }}
        >
          aaa
        </button>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
};

export default App;
