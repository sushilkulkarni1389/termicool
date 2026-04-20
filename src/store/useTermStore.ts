import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { invoke } from "@tauri-apps/api/core";

export interface Colors {
  background: string;
  foreground: string;
  cursor: string;
  selection: string;
  black: string;
  red: string;
  green: string;
  yellow: string;
  blue: string;
  magenta: string;
  cyan: string;
  white: string;
  brightBlack: string;
  brightRed: string;
  brightGreen: string;
  brightYellow: string;
  brightBlue: string;
  brightMagenta: string;
  brightCyan: string;
  brightWhite: string;
}

export interface Theme {
  name: string;
  colors: Colors;
  id?: string;
}

interface TermState {
  theme: Theme | null;
  activeThemeName: string;
  savedThemes: Theme[];
  promptModules: string[];
  setTheme: (theme: Theme) => void;
  updateColor: (key: keyof Colors, value: string) => void;
  togglePromptModule: (module: string) => void;
  loadInitialTheme: () => Promise<void>;
  saveTheme: (name: string) => Promise<void>;
  loadAllThemes: () => Promise<void>;
}

export const useTermStore = create<TermState>()(
  persist(
    (set, get) => ({
      theme: null,
      activeThemeName: "Default",
      savedThemes: [],
      promptModules: ['directory', 'git_branch', 'nodejs'],
      
      setTheme: (theme) => set({ theme, activeThemeName: theme.name }),
      
      updateColor: (key, value) => {
        const { theme } = get();
        if (!theme) return;
        set({
          theme: {
            ...theme,
            colors: { ...theme.colors, [key]: value }
          }
        });
      },

      togglePromptModule: async (module) => {
        const { promptModules } = get();
        const newModules = promptModules.includes(module)
          ? promptModules.filter(m => m !== module)
          : [...promptModules, module];
        
        set({ promptModules: newModules });
        
        try {
          await invoke("generate_prompt", { modules: newModules });
        } catch (e) {
          console.error("Failed to generate prompt:", e);
        }
      },

      loadInitialTheme: async () => {
        // If we already have a theme from persistence, just load the list of themes
        const state = get();
        if (state.theme) {
          await state.loadAllThemes();
          return;
        }

        try {
          const theme = await invoke<Theme>("load_theme", { name: "default" });
          set({ theme, activeThemeName: theme.name });
          await state.loadAllThemes();
        } catch (e) {
          console.error("Failed to load theme:", e);
        }
      },

      saveTheme: async (name) => {
        const { theme, loadAllThemes } = get();
        if (!theme) return;
        const newTheme = { ...theme, name };
        try {
          await invoke("save_theme", { name, theme: newTheme });
          set({ activeThemeName: name, theme: newTheme });
          await loadAllThemes();
        } catch (e) {
          console.error("Failed to save theme:", e);
          throw e;
        }
      },

      loadAllThemes: async () => {
        try {
          const themes = await invoke<Theme[]>("load_themes");
          set({ savedThemes: themes });
        } catch (e) {
          console.error("Failed to load all themes:", e);
        }
      }
    }),
    {
      name: 'termicool-storage',
      storage: createJSONStorage(() => localStorage),
      // We only want to persist the UI state, not the full list of themes which is loaded from disk
      partialize: (state) => ({ 
        theme: state.theme, 
        activeThemeName: state.activeThemeName, 
        promptModules: state.promptModules 
      }),
    }
  )
);
