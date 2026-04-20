import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { platform } from "@tauri-apps/plugin-os";
import { useTermStore, Colors, Theme } from "./store/useTermStore";
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
    setTheme,
    loadAllThemes
  } = useTermStore();
  
  const [activeTab, setActiveTab] = useState<"theme" | "prompt" | "settings">("theme");
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isInitializing, setIsInitializing] = useState(true);
  const [applyToIdes, setApplyToIdes] = useState<boolean>(
    () => localStorage.getItem("termicool_apply_to_ides") === "true"
  );
  const [hexDrafts, setHexDrafts] = useState<Partial<Record<keyof Colors, string>>>({});
  const [showCreator, setShowCreator] = useState(false);
  const [creatorTheme, setCreatorTheme] = useState<Theme | null>(null);
  const [creatorName, setCreatorName] = useState("");
  const [creatorHexDrafts, setCreatorHexDrafts] = useState<Partial<Record<keyof Colors, string>>>({});
  const [creatorSaveError, setCreatorSaveError] = useState<string | null>(null);
  const [creatorIsSaving, setCreatorIsSaving] = useState(false);
  const [themeSearch, setThemeSearch] = useState("");
  const filteredThemes = [...savedThemes]
    .sort((a, b) => a.name.localeCompare(b.name))
    .filter(t => t.name.toLowerCase().includes(themeSearch.toLowerCase()));

  useEffect(() => {
    localStorage.setItem("termicool_apply_to_ides", String(applyToIdes));
  }, [applyToIdes]);
  
  // System State
  const [isFontInstalled, setIsFontInstalled] = useState(false);
  const [isStarshipInstalled, setIsStarshipInstalled] = useState(false);
  const [isStarshipLoading, setIsStarshipLoading] = useState(false);
  const [isCliInstalled, setIsCliInstalled] = useState(false);
  const [isCliLoading, setIsCliLoading] = useState(false);
  const [isDefaultState, setIsDefaultState] = useState(false);

  useEffect(() => {
    async function init() {
      try {
        await loadInitialTheme();
        await refreshSystemState();
      } catch (e) {
        window.alert("LINUX STARTUP ERROR: " + JSON.stringify(e));
        console.error("Initialization failed", e);
      } finally {
        setIsInitializing(false);
      }
    }
    init();
  }, []);

  async function refreshSystemState() {
    try {
      const fontStatus = await invoke<boolean>("check_font_installed");
      const starshipStatus = await invoke<boolean>("check_starship_installed");
      invoke<boolean>("check_cli_installed").then(setIsCliInstalled);
      const defaultStatus = await invoke<boolean>("check_is_default");
      setIsFontInstalled(fontStatus);
      setIsStarshipInstalled(starshipStatus);
      setIsDefaultState(defaultStatus);
    } catch (e) {
      window.alert("LINUX STARTUP ERROR (refresh): " + JSON.stringify(e));
      console.error("Failed to check system state", e);
    }
  }

  async function applyTheme() {
    if (!theme) return;
    try {
      await invoke<string>("apply_theme", { theme });
      await invoke<string>("generate_prompt", { modules: promptModules });
      const currentPlatform = platform();
      if (currentPlatform === "linux" || currentPlatform === "windows") {
        setStatus("Theme and prompt applied! Open a new terminal window to see the changes.");
      } else {
        setStatus("Theme and prompt applied!");
      }
      refreshSystemState();
      if (applyToIdes) {
        invoke<string[]>("apply_theme_to_ides", { theme })
          .then((ides: string[]) => {
            if (ides.length > 0) {
              setStatus(`Theme applied! Also applied to: ${ides.join(", ")}`);
            }
          })
          .catch((err: string) => {
            console.error("IDE apply failed:", err);
          });
      }
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
    try {
      const response = await invoke<string>("revert_to_default");
      setStatus(response);
      // Reset sidebar selection to "TermiCool Default"
      const tcDefault = savedThemes.find(t => t.name === "TermiCool Default");
      if (tcDefault) {
        setTheme(tcDefault);
      } else {
        // Fallback: load from Rust (returns hardcoded default when file is gone)
        const defaultTheme = await invoke<Theme>("load_theme", { name: "termicool_default" });
        setTheme(defaultTheme);
      }
      await refreshSystemState();
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleStarshipInstall() {
    setIsStarshipLoading(true);
    setStatus("Downloading Starship...");
    try {
      const msg = await invoke<string>("install_starship");
      setStatus(msg);
      await refreshSystemState();
    } catch (e) {
      setError(String(e));
    } finally {
      setIsStarshipLoading(false);
    }
  }

  const handleCliInstall = async () => {
    setIsCliLoading(true);
    try {
      const msg = await invoke<string>("install_cli");
      setIsCliInstalled(true);
      setStatus(msg);
    } catch (e) {
      setError("CLI install failed: " + e);
    } finally {
      setIsCliLoading(false);
    }
  };

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

  function openCreator(baseTheme?: Theme) {
    const base = baseTheme || savedThemes[0] || theme;
    if (!base) return;
    setCreatorTheme({ ...base, name: "" });
    setCreatorName("");
    setCreatorHexDrafts({});
    setCreatorSaveError(null);
    setShowCreator(true);
  }

  function updateCreatorColor(key: keyof Colors, value: string) {
    if (!creatorTheme) return;
    setCreatorTheme({
      ...creatorTheme,
      colors: { ...creatorTheme.colors, [key]: value }
    });
  }

  async function handleCreatorSave() {
    if (!creatorTheme) return;
    const trimmed = creatorName.trim();
    if (!trimmed) {
      setCreatorSaveError("Please enter a theme name.");
      return;
    }
    setCreatorIsSaving(true);
    setCreatorSaveError(null);
    try {
      const themeToSave = { ...creatorTheme, name: trimmed };
      await invoke("save_theme", { name: trimmed, theme: themeToSave });
      await loadAllThemes();
      setShowCreator(false);
      setStatus(`Theme '${trimmed}' saved.`);
    } catch (e) {
      setCreatorSaveError(String(e));
    } finally {
      setCreatorIsSaving(false);
    }
  }

  function handleCreatorExport() {
    if (!creatorTheme) return;
    const trimmed = creatorName.trim() || "custom-theme";
    const themeToExport = { ...creatorTheme, name: trimmed };
    const blob = new Blob(
      [JSON.stringify(themeToExport, null, 2)],
      { type: "application/json" }
    );
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${trimmed.toLowerCase().replace(/\s+/g, "_")}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  if (isInitializing) return <div className="loading">Loading...</div>;

  const coreColors: (keyof Colors)[] = ["background", "foreground", "cursor", "selection"];
  const normalANSI: (keyof Colors)[] = ["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];
  const brightANSI: (keyof Colors)[] = ["brightBlack", "brightRed", "brightGreen", "brightYellow", "brightBlue", "brightMagenta", "brightCyan", "brightWhite"];

  const commonModules = ["directory", "git_branch", "nodejs", "python", "rust", "java", "cmd_duration", "time"];

  return (
    <main className="container">
      <div className="main-layout">
        <aside className="sidebar">
          <h3>Saved Themes</h3>
          <div className="sidebar-search-wrapper">
            <input
              type="text"
              className="sidebar-search"
              placeholder="Search themes..."
              value={themeSearch}
              onChange={e => setThemeSearch(e.target.value)}
            />
            {themeSearch && (
              <button className="sidebar-search-clear" onClick={() => setThemeSearch("")}>
                ✕
              </button>
            )}
          </div>
          <div className="theme-list">
            {filteredThemes.length > 0 ? (
              filteredThemes.map((t) => (
                <button
                  key={t.name}
                  className={`theme-item ${activeThemeName === t.name ? 'active' : ''}`}
                  onClick={() => setTheme(t)}
                >
                  {t.name}
                </button>
              ))
            ) : (
              <p className="sidebar-no-results">No themes found.</p>
            )}
          </div>
          <button className="new-theme-btn" onClick={() => openCreator()}>
            + New Theme
          </button>
        </aside>

        <section className="content-area">
          <nav className="tab-nav">
            <button className={activeTab === "theme" ? "active" : ""} onClick={() => setActiveTab("theme")}>Theme</button>
            <button className={activeTab === "prompt" ? "active" : ""} onClick={() => setActiveTab("prompt")}>Prompt</button>
            <button className={activeTab === "settings" ? "active" : ""} onClick={() => setActiveTab("settings")}>Settings</button>
          </nav>

          {activeTab === "theme" && theme ? (
            <div className="tab-content theme-layout">
              <div className="theme-editor-left">
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
                          <input
                            type="color"
                            value={theme.colors[key]}
                            onChange={(e) => {
                              updateColor(key, e.target.value);
                              setHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                            }}
                          />
                          <input
                            type="text"
                            className={`hex-input${hexDrafts[key] !== undefined && !/^#[0-9a-fA-F]{6}$/.test(hexDrafts[key]!) ? ' invalid' : ''}`}
                            value={hexDrafts[key] ?? theme.colors[key]}
                            maxLength={7}
                            spellCheck={false}
                            onChange={(e) => {
                              const val = e.target.value;
                              setHexDrafts(d => ({ ...d, [key]: val }));
                              if (/^#[0-9a-fA-F]{6}$/.test(val)) {
                                updateColor(key, val);
                                setHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                              }
                            }}
                            onBlur={() => {
                              setHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                            }}
                          />
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
                          <input
                            type="color"
                            value={theme.colors[key]}
                            onChange={(e) => {
                              updateColor(key, e.target.value);
                              setHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                            }}
                          />
                          <input
                            type="text"
                            className={`hex-input${hexDrafts[key] !== undefined && !/^#[0-9a-fA-F]{6}$/.test(hexDrafts[key]!) ? ' invalid' : ''}`}
                            value={hexDrafts[key] ?? theme.colors[key]}
                            maxLength={7}
                            spellCheck={false}
                            onChange={(e) => {
                              const val = e.target.value;
                              setHexDrafts(d => ({ ...d, [key]: val }));
                              if (/^#[0-9a-fA-F]{6}$/.test(val)) {
                                updateColor(key, val);
                                setHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                              }
                            }}
                            onBlur={() => {
                              setHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                            }}
                          />
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
                          <input
                            type="color"
                            value={theme.colors[key]}
                            onChange={(e) => {
                              updateColor(key, e.target.value);
                              setHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                            }}
                          />
                          <input
                            type="text"
                            className={`hex-input${hexDrafts[key] !== undefined && !/^#[0-9a-fA-F]{6}$/.test(hexDrafts[key]!) ? ' invalid' : ''}`}
                            value={hexDrafts[key] ?? theme.colors[key]}
                            maxLength={7}
                            spellCheck={false}
                            onChange={(e) => {
                              const val = e.target.value;
                              setHexDrafts(d => ({ ...d, [key]: val }));
                              if (/^#[0-9a-fA-F]{6}$/.test(val)) {
                                updateColor(key, val);
                                setHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                              }
                            }}
                            onBlur={() => {
                              setHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                            }}
                          />
                        </div>
                      ))}
                    </div>
                  </section>
                </div>
                <div className="actions">
                  <button className="primary" onClick={applyTheme}>Apply Theme</button>
                  <label className="module-toggle">
                    <input
                      type="checkbox"
                      checked={applyToIdes}
                      onChange={e => setApplyToIdes(e.target.checked)}
                    />
                    <span className="toggle-label">Apply to VS Code / Cursor</span>
                  </label>
                </div>
              </div>
              <div className="theme-editor-right">
                <p className="preview-label">Terminal Preview</p>
                <div className="terminal-preview" style={{ 
                  backgroundColor: theme?.colors.background || "#1a1a1a", 
                  color: theme?.colors.foreground || "#fff",
                }}>
                  <div className="terminal-header">Terminal Preview</div>
                  <div className="terminal-body">
                    <p>$ ls -la</p>
                    <p><span style={{ color: theme?.colors.blue || "blue" }}>drwxr-xr-x</span>  5 user  staff  160 Apr  9 12:00 .</p>
                    <p><span style={{ color: theme?.colors.green || "green" }}>-rw-r--r--</span>  1 user  staff    0 Apr  9 12:00 theme.json</p>
                    <p><span style={{ color: theme?.colors.red || "red" }}>[ERROR]</span> something went wrong</p>
                    <p className="cursor-line">
                      <span style={{ color: theme?.colors.magenta || "magenta" }}>~/Documents/TermiCool</span> 
                      <span style={{ color: theme?.colors.cyan || "cyan" }}> on  main</span>
                    </p>
                  </div>
                </div>
                <div className="color-strip">
                  {(["black","red","green","yellow","blue","magenta","cyan","white"] as (keyof Colors)[]).map(key => (
                    <div
                      key={key}
                      className="strip-swatch"
                      style={{ backgroundColor: theme.colors[key] }}
                      title={key}
                    />
                  ))}
                </div>
              </div>
            </div>
          ) : activeTab === "theme" ? (
            <div className="tab-content">
              <h1>Theme not loaded</h1>
              <p>Try refreshing or checking system settings.</p>
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
                  <p>Download and install MesloLGS NF {platform() === "macos" ? "" : "Mono "}Regular for the best prompt experience.</p>
                  <button 
                    onClick={handleFontInstall} 
                    disabled={isLoading || isFontInstalled}
                    className={isFontInstalled ? "success-btn" : ""}
                  >
                    {isLoading ? "Installing..." : isFontInstalled ? "Nerd Font Installed ✓" : `Install Meslo Nerd Font${platform() === "macos" ? "" : " Mono"}`}
                  </button>
                  <p className="helper-text">
                    Note: You must manually set your terminal's font preference to MesloLGS NF {platform() === "macos" ? "" : "Mono "}after installing.
                  </p>
                </div>
                
                {platform() !== "macos" && (
                  <div className="settings-card">
                    <h3>Starship Prompt Installer</h3>
                    <p>Download and install the Starship prompt binary for a customisable terminal prompt.</p>
                    <button
                      onClick={handleStarshipInstall}
                      disabled={isStarshipLoading || isStarshipInstalled}
                      className={isStarshipInstalled ? "success-btn" : ""}
                    >
                      {isStarshipLoading ? "Installing..." : isStarshipInstalled ? "Starship Installed ✓" : "Install Starship Prompt"}
                    </button>
                    <p className="helper-text">
                      Note: Open a new terminal window after installing to activate the prompt.
                    </p>
                  </div>
                )}

                <div className="settings-card">
                  <h3>CLI Installer</h3>
                  <p>
                    Install the <code>termicool</code> CLI to your PATH so you can
                    apply themes from any terminal window.
                  </p>
                  <button
                    onClick={handleCliInstall}
                    disabled={isCliLoading || isCliInstalled}
                    className={isCliInstalled ? "success-btn" : ""}
                  >
                    {isCliLoading
                      ? "Installing..."
                      : isCliInstalled
                      ? "CLI Installed ✓"
                      : "Install CLI"}
                  </button>
                  <p className="helper-text">
                    Installs to /usr/local/bin/termicool and sets up tab completion
                    for your shell. Restart your terminal after installing.
                  </p>
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

          {showCreator && creatorTheme && (
            <div className="creator-backdrop" onClick={(e) => { if (e.target === e.currentTarget) setShowCreator(false); }}>
              <div className="creator-modal">

                <div className="creator-header">
                  <div>
                    <p className="creator-title">Create custom theme</p>
                    <p className="creator-subtitle">Saved to ~/.termicool/themes/ and available in sidebar</p>
                  </div>
                  <button className="creator-close" onClick={() => setShowCreator(false)}>✕</button>
                </div>

                <div className="creator-body">

                  <div className="creator-left">
                    <div className="creator-field-group">
                      <label className="creator-label">Theme name</label>
                      <input
                        type="text"
                        className={`creator-name-input${creatorSaveError && !creatorName.trim() ? ' invalid' : ''}`}
                        placeholder="My awesome theme"
                        value={creatorName}
                        onChange={e => { setCreatorName(e.target.value); setCreatorSaveError(null); }}
                      />
                    </div>

                    <div className="creator-field-group">
                      <label className="creator-label">Start from existing theme</label>
                      <select
                        className="creator-select"
                        onChange={e => {
                          const base = savedThemes.find(t => t.name === e.target.value);
                          if (base) setCreatorTheme({ ...base, name: creatorName });
                        }}
                      >
                        {savedThemes.map(t => (
                          <option key={t.name} value={t.name}>{t.name}</option>
                        ))}
                      </select>
                    </div>

                    <div className="creator-section-title">Base</div>
                    <div className="creator-color-grid">
                      {(["background","foreground","cursor","selection"] as (keyof Colors)[]).map(key => (
                        <div key={key} className="field">
                          <label title={key}>{key}</label>
                          <input
                            type="color"
                            value={creatorTheme.colors[key]}
                            onChange={(e) => {
                              updateCreatorColor(key, e.target.value);
                              setCreatorHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                            }}
                          />
                          <input
                            type="text"
                            className={`hex-input${creatorHexDrafts[key] !== undefined && !/^#[0-9a-fA-F]{6}$/.test(creatorHexDrafts[key]!) ? ' invalid' : ''}`}
                            value={creatorHexDrafts[key] ?? creatorTheme.colors[key]}
                            maxLength={7}
                            spellCheck={false}
                            onChange={(e) => {
                              const val = e.target.value;
                              setCreatorHexDrafts(d => ({ ...d, [key]: val }));
                              if (/^#[0-9a-fA-F]{6}$/.test(val)) {
                                updateCreatorColor(key, val);
                                setCreatorHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                              }
                            }}
                            onBlur={() => setCreatorHexDrafts(d => { const n = { ...d }; delete n[key]; return n; })}
                          />
                        </div>
                      ))}
                    </div>

                    <div className="creator-section-title">ANSI colors</div>
                    <div className="creator-color-grid">
                      {(["black","red","green","yellow","blue","magenta","cyan","white"] as (keyof Colors)[]).map(key => (
                        <div key={key} className="field">
                          <label title={key}>{key}</label>
                          <input
                            type="color"
                            value={creatorTheme.colors[key]}
                            onChange={(e) => {
                              updateCreatorColor(key, e.target.value);
                              setCreatorHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                            }}
                          />
                          <input
                            type="text"
                            className={`hex-input${creatorHexDrafts[key] !== undefined && !/^#[0-9a-fA-F]{6}$/.test(creatorHexDrafts[key]!) ? ' invalid' : ''}`}
                            value={creatorHexDrafts[key] ?? creatorTheme.colors[key]}
                            maxLength={7}
                            spellCheck={false}
                            onChange={(e) => {
                              const val = e.target.value;
                              setCreatorHexDrafts(d => ({ ...d, [key]: val }));
                              if (/^#[0-9a-fA-F]{6}$/.test(val)) {
                                updateCreatorColor(key, val);
                                setCreatorHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                              }
                            }}
                            onBlur={() => setCreatorHexDrafts(d => { const n = { ...d }; delete n[key]; return n; })}
                          />
                        </div>
                      ))}
                    </div>

                    <div className="creator-section-title">ANSI bright</div>
                    <div className="creator-color-grid">
                      {(["brightBlack","brightRed","brightGreen","brightYellow","brightBlue","brightMagenta","brightCyan","brightWhite"] as (keyof Colors)[]).map(key => (
                        <div key={key} className="field">
                          <label title={key}>{key}</label>
                          <input
                            type="color"
                            value={creatorTheme.colors[key]}
                            onChange={(e) => {
                              updateCreatorColor(key, e.target.value);
                              setCreatorHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                            }}
                          />
                          <input
                            type="text"
                            className={`hex-input${creatorHexDrafts[key] !== undefined && !/^#[0-9a-fA-F]{6}$/.test(creatorHexDrafts[key]!) ? ' invalid' : ''}`}
                            value={creatorHexDrafts[key] ?? creatorTheme.colors[key]}
                            maxLength={7}
                            spellCheck={false}
                            onChange={(e) => {
                              const val = e.target.value;
                              setCreatorHexDrafts(d => ({ ...d, [key]: val }));
                              if (/^#[0-9a-fA-F]{6}$/.test(val)) {
                                updateCreatorColor(key, val);
                                setCreatorHexDrafts(d => { const n = { ...d }; delete n[key]; return n; });
                              }
                            }}
                            onBlur={() => setCreatorHexDrafts(d => { const n = { ...d }; delete n[key]; return n; })}
                          />
                        </div>
                      ))}
                    </div>
                  </div>

                  <div className="creator-right">
                    <p className="preview-label">Live preview</p>
                    <div className="terminal-preview" style={{
                      backgroundColor: creatorTheme.colors.background,
                      color: creatorTheme.colors.foreground,
                    }}>
                      <div className="terminal-header">Terminal Preview</div>
                      <div className="terminal-body">
                        <p>$ ls -la</p>
                        <p><span style={{ color: creatorTheme.colors.blue }}>drwxr-xr-x</span>  5 user  staff  160 Apr  9 12:00 .</p>
                        <p><span style={{ color: creatorTheme.colors.green }}>-rw-r--r--</span>  1 user  staff    0 Apr  9 12:00 theme.json</p>
                        <p><span style={{ color: creatorTheme.colors.red }}>[ERROR]</span> something went wrong</p>
                        <p className="cursor-line">
                          <span style={{ color: creatorTheme.colors.magenta }}>~/Documents/TermiCool</span>
                          <span style={{ color: creatorTheme.colors.cyan }}> on  main</span>
                        </p>
                      </div>
                    </div>
                    <div className="color-strip">
                      {(["black","red","green","yellow","blue","magenta","cyan","white"] as (keyof Colors)[]).map(key => (
                        <div
                          key={key}
                          className="strip-swatch"
                          style={{ backgroundColor: creatorTheme.colors[key] }}
                          title={key}
                        />
                      ))}
                    </div>
                  </div>

                </div>

                {creatorSaveError && (
                  <p className="creator-error">{creatorSaveError}</p>
                )}

                <div className="creator-footer">
                  <button className="creator-export-btn" onClick={handleCreatorExport}>
                    Export .json
                  </button>
                  <div className="creator-footer-right">
                    <button className="creator-cancel-btn" onClick={() => setShowCreator(false)}>
                      Cancel
                    </button>
                    <button
                      className="creator-save-btn"
                      onClick={handleCreatorSave}
                      disabled={creatorIsSaving}
                    >
                      {creatorIsSaving ? "Saving..." : "Save theme"}
                    </button>
                  </div>
                </div>

              </div>
            </div>
          )}
        </section>
      </div>
    </main>
  );
}

export default App;
