import { writable } from 'svelte/store';
import { THEME, THEME_COLORS } from '../config';

// Theme type
export type ThemeType = typeof THEME.LIGHT | typeof THEME.DARK;

// Theme store interface
interface ThemeStore {
  currentTheme: ThemeType;
  colors: typeof THEME_COLORS[ThemeType];
}

// Get initial theme from localStorage or system preference
const getInitialTheme = (): ThemeType => {
  if (typeof window === 'undefined') {
    return THEME.LIGHT;
  }

  const storedTheme = localStorage.getItem('theme') as ThemeType;
  if (storedTheme && (storedTheme === THEME.LIGHT || storedTheme === THEME.DARK)) {
    return storedTheme;
  }

  // Check system preference
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return THEME.DARK;
  }

  return THEME.LIGHT;
};

// Create theme store
const createThemeStore = () => {
  const initialTheme = getInitialTheme();
  const { subscribe, update, set } = writable<ThemeStore>({
    currentTheme: initialTheme,
    colors: THEME_COLORS[initialTheme],
  });

  // Apply theme to the document
  const applyTheme = (theme: ThemeType) => {
    if (typeof document !== 'undefined') {
      document.documentElement.setAttribute('data-theme', theme);
      
      // Set CSS variables for the theme colors
      Object.entries(THEME_COLORS[theme]).forEach(([key, value]) => {
        document.documentElement.style.setProperty(`--color-${key}`, value);
      });
    }
  };

  return {
    subscribe,
    
    // Initialize theme on app start
    initialize: () => {
      const theme = getInitialTheme();
      applyTheme(theme);
      set({
        currentTheme: theme,
        colors: THEME_COLORS[theme],
      });
    },
    
    // Toggle between light and dark themes
    toggle: () => {
      update(state => {
        const newTheme = state.currentTheme === THEME.LIGHT ? THEME.DARK : THEME.LIGHT;
        
        // Store preference in localStorage
        if (typeof localStorage !== 'undefined') {
          localStorage.setItem('theme', newTheme);
        }
        
        // Apply the theme
        applyTheme(newTheme);
        
        return {
          currentTheme: newTheme,
          colors: THEME_COLORS[newTheme],
        };
      });
    },
    
    // Set a specific theme
    setTheme: (theme: ThemeType) => {
      // Only update if it's a different theme
      update(state => {
        if (state.currentTheme === theme) {
          return state;
        }
        
        // Store preference in localStorage
        if (typeof localStorage !== 'undefined') {
          localStorage.setItem('theme', theme);
        }
        
        // Apply the theme
        applyTheme(theme);
        
        return {
          currentTheme: theme,
          colors: THEME_COLORS[theme],
        };
      });
    },
  };
};

// Create and export the store
export const themeStore = createThemeStore(); 