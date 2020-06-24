import React from "react";
import { open } from "tauri/api/dialog";
import logo from "./logo.svg";
import "./App.css";
import API from "./tauriAPI";

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
          onClick={async (): Promise<void> => {
            const path = await open();
            console.log(await API.getTweetCount({ path }));
          }}
        >
          bbb
        </button>
      </header>
    </div>
  );
};

export default App;
