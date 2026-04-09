import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useTermStore, Colors } from "./store/useTermStore";
import "./App.css";

function App() {
  const { 
    theme, 
    activeThemeName,
    savedThemes, 
    promptModules, 
    loadInitialTheme, 
    updateColor, 
    togglePromptModule,
    saveTheme,
    setTheme
  } = useTermStore();
  
  const [activeTab, setActiveTab] = useState<"theme" | "prompt" | "settings">("theme");
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  
  // System State
  const [isFontInstalled, setIsFontInstalled] = useState(false);
  const [isDefaultState, setIsDefaultState] = useState(false);

  useEffect(() => {
    loadInitialTheme();
    refreshSystemState();
  }, []);

  async function refreshSystemState() {
    try {
      const fontStatus = await invoke<boolean>("check_font_installed");
      const defaultStatus = await invoke<boolean>("check_is_default");
      setIsFontInstalled(fontStatus);
      setIsDefaultState(defaultStatus);
    } catch (e) {
      console.error("Failed to check system state", e);
    }
  }

  async function applyTheme() {
    if (!theme) return;
    try {
      await invoke<string>("apply_theme", { theme });
      setStatus("Theme applied!");
      refreshSystemState();
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleSave() {
    const name = prompt("Enter theme name:");
    if (name) {
      try {
        await saveTheme(name);
        setStatus(`Theme '${name}' saved.`);
      } catch (e) {
        setError("Failed to save theme");
      }
    }
  }

  // async function handleRevert() {
  //   window.alert("DEBUG 1: React button clicked! Preparing to invoke...");
  //   try {
  //     const response = await invoke<string>("revert_to_default");
  //     window.alert("DEBUG 2: Rust responded with SUCCESS: " + response);
  //     await refreshSystemState();
  //   } catch (error) {
  //     window.alert("DEBUG 3: Tauri Invoke FAILED: " + JSON.stringify(error));
  //   }
  // }

  async function handleRevert() {
    console.log(">>> REACT: handleRevert start");
    alert("React: Starting Revert");
    try {
      const response = await invoke<string>("revert_to_default");
      alert("React: Success - " + response);
      console.log(">>> REACT: Success", response);
      await refreshSystemState();
    } catch (e) {
      alert("React: Error - " + e);
      console.error(">>> REACT: Error", e);
    }
  }

  async function handleFontInstall() {
    setIsLoading(true);
    setStatus("Downloading font...");
    try {
      const msg = await invoke<string>("download_font");
      setStatus(msg);
      await refreshSystemState();
    } catch (e) {
      setError(String(e));
    } finally {
      setIsLoading(false);
    }
  }

  if (!theme) return <div className="loading">Loading...</div>;

  const coreColors: (keyof Colors)[] = ["background", "foreground", "cursor", "selection"];
  const normalANSI: (keyof Colors)[] = ["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];
  const brightANSI: (keyof Colors)[] = ["brightBlack", "brightRed", "brightGreen", "brightYellow", "brightBlue", "brightMagenta", "brightCyan", "brightWhite"];

  const commonModules = ["directory", "git_branch", "nodejs", "python", "rust", "java", "cmd_duration", "time"];

  return (
    <main className="container">
      <div className="main-layout">
        <aside className="sidebar">
          <h3>Saved Themes</h3>
          <div className="theme-list">
            {savedThemes.map((t) => (
              <button 
                key={t.name} 
                className={`theme-item ${activeThemeName === t.name ? 'active' : ''}`}
                onClick={() => setTheme(t)}
              >
                {t.name}
              </button>
            ))}
          </div>
        </aside>

        <section className="content-area">
          <nav className="tab-nav">
            <button className={activeTab === "theme" ? "active" : ""} onClick={() => setActiveTab("theme")}>Theme</button>
            <button className={activeTab === "prompt" ? "active" : ""} onClick={() => setActiveTab("prompt")}>Prompt</button>
            <button className={activeTab === "settings" ? "active" : ""} onClick={() => setActiveTab("settings")}>Settings</button>
          </nav>

          {activeTab === "theme" ? (
            <div className="tab-content">
              <div className="header-row">
                <h1>Theme Editor: {activeThemeName}</h1>
                <button onClick={handleSave}>Save Theme</button>
              </div>
              <div className="editor-grid">
                <section>
                  <h3>Core</h3>
                  <div className="color-grid">
                    {coreColors.map(key => (
                      <div key={key} className="field">
                        <label title={key}>{key}</label>
                        <input type="color" value={theme.colors[key]} onChange={(e) => updateColor(key, e.target.value)} />
                      </div>
                    ))}
                  </div>
                </section>
                <section>
                  <h3>ANSI Normal</h3>
                  <div className="color-grid">
                    {normalANSI.map(key => (
                      <div key={key} className="field">
                        <label title={key}>{key}</label>
                        <input type="color" value={theme.colors[key]} onChange={(e) => updateColor(key, e.target.value)} />
                      </div>
                    ))}
                  </div>
                </section>
                <section>
                  <h3>ANSI Bright</h3>
                  <div className="color-grid">
                    {brightANSI.map(key => (
                      <div key={key} className="field">
                        <label title={key}>{key}</label>
                        <input type="color" value={theme.colors[key]} onChange={(e) => updateColor(key, e.target.value)} />
                      </div>
                    ))}
                  </div>
                </section>
              </div>
              <div className="actions">
                <button className="primary" onClick={applyTheme}>Apply Theme</button>
              </div>
            </div>
          ) : activeTab === "prompt" ? (
            <div className="tab-content">
              <h1>Prompt Generator</h1>
              <p>Toggle modules to dynamically update your <code>starship.toml</code></p>
              <div className="prompt-grid">
                {commonModules.map(module => (
                  <label key={module} className="module-toggle">
                    <input 
                      type="checkbox" 
                      checked={promptModules.includes(module)} 
                      onChange={() => togglePromptModule(module)} 
                    />
                    <span className="toggle-label">{module}</span>
                  </label>
                ))}
              </div>
              <div className="preview prompt-preview">
                <p className="label">Preview:</p>
                <code>
                  {promptModules.map(m => `$${m}`).join(" ")}
                </code>
              </div>
            </div>
          ) : (
            <div className="tab-content">
              <h1>System Settings</h1>
              <div className="settings-grid">
                <div className="settings-card">
                  <h3>Nerd Font Installer</h3>
                  <p>Download and install MesloLGS NF Regular for the best prompt experience.</p>
                  <button 
                    onClick={handleFontInstall} 
                    disabled={isLoading || isFontInstalled}
                    className={isFontInstalled ? "success-btn" : ""}
                  >
                    {isLoading ? "Installing..." : isFontInstalled ? "Nerd Font Installed ✓" : "Install Meslo Nerd Font"}
                  </button>
                </div>
                
                <div className="settings-card danger">
                  <h3>Failsafe</h3>
                  <p>Restore your shell profile and terminal settings from original backups.</p>
                  <button 
                    className={`revert ${isDefaultState ? 'muted' : ''}`} 
                    onClick={handleRevert}
                    disabled={isDefaultState}
                  >
                    {isDefaultState ? "Terminal is in Default State" : "Emergency Revert to Default"}
                  </button>
                </div>
              </div>
            </div>
          )}

          {(error || status) && (
            <div className={`notification ${error ? 'error' : 'success'}`}>
              {error || status}
              <button onClick={() => { setError(null); setStatus(null); }}>×</button>
            </div>
          )}
          
          <div className="terminal-preview" style={{ 
            backgroundColor: theme.colors.background, 
            color: theme.colors.foreground,
          }}>
            <div className="terminal-header">Terminal Preview</div>
            <div className="terminal-body">
              <p>$ ls -la</p>
              <p><span style={{ color: theme.colors.blue }}>drwxr-xr-x</span>  5 user  staff  160 Apr  9 12:00 .</p>
              <p><span style={{ color: theme.colors.green }}>-rw-r--r--</span>  1 user  staff    0 Apr  9 12:00 theme.json</p>
              <p><span style={{ color: theme.colors.red }}>[ERROR]</span> something went wrong</p>
              <p className="cursor-line">
                <span style={{ color: theme.colors.magenta }}>~/Documents/TermiCool</span> 
                <span style={{ color: theme.colors.cyan }}> on  main</span>
              </p>
            </div>
          </div>
        </section>
      </div>
    </main>
  );
}

export default App;
