// Configuration for the frontend application

// API URL for backend communication
export const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8000/api';

// WebSocket URL for real-time features
export const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8000/ws';

// Authentication token storage key in localStorage
export const TOKEN_KEY = 'dev_forum_token';

// Theme constants
export const THEME = {
  LIGHT: 'light',
  DARK: 'dark'
};

// Theme colors
export const THEME_COLORS = {
  [THEME.LIGHT]: {
    primary: '#F2F0E3',
    secondary: '#2E2E2E',
    text: '#2E2E2E',
    textSecondary: '#4A4A4A',
    background: '#FFFFFF',
    card: '#F8F8F8',
    border: '#E0E0E0',
    accent: '#4A87C7',
    success: '#4CAF50',
    warning: '#FF9800',
    error: '#E53935',
  },
  [THEME.DARK]: {
    primary: '#1F1F1F',
    secondary: '#D1CFC0',
    text: '#D1CFC0',
    textSecondary: '#A0A0A0',
    background: '#121212',
    card: '#2A2A2A',
    border: '#3A3A3A',
    accent: '#5D93CC',
    success: '#43A047',
    warning: '#FB8C00',
    error: '#E53935',
  }
};

// Maximum file size for uploads (in bytes)
export const MAX_UPLOAD_SIZE = 20 * 1024 * 1024; // 20MB

// Supported code languages for syntax highlighting
export const CODE_LANGUAGES = [
  { value: 'javascript', label: 'JavaScript' },
  { value: 'typescript', label: 'TypeScript' },
  { value: 'python', label: 'Python' },
  { value: 'rust', label: 'Rust' },
  { value: 'go', label: 'Go' },
  { value: 'java', label: 'Java' },
  { value: 'c', label: 'C' },
  { value: 'cpp', label: 'C++' },
  { value: 'csharp', label: 'C#' },
  { value: 'php', label: 'PHP' },
  { value: 'ruby', label: 'Ruby' },
  { value: 'html', label: 'HTML' },
  { value: 'css', label: 'CSS' },
  { value: 'json', label: 'JSON' },
  { value: 'xml', label: 'XML' },
  { value: 'markdown', label: 'Markdown' },
  { value: 'sql', label: 'SQL' },
  { value: 'bash', label: 'Bash' },
  { value: 'plaintext', label: 'Plain Text' },
];

// Platform options for file sharing
export const PLATFORMS = [
  { value: 'windows', label: 'Windows' },
  { value: 'macos', label: 'macOS' },
  { value: 'linux', label: 'Linux' },
  { value: 'cross-platform', label: 'Cross-Platform' },
  { value: 'android', label: 'Android' },
  { value: 'ios', label: 'iOS' },
  { value: 'other', label: 'Other' },
]; 