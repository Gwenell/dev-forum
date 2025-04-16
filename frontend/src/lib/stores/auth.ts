import { writable } from 'svelte/store';
import { api, setToken, removeToken, isAuthenticated } from '$lib/api';
import type { User } from '$lib/types';

// User interface
export interface User {
  id: string;
  username: string;
  email: string;
  display_name?: string;
  avatar_url?: string;
  theme_preference?: string;
  bio?: string;
  is_admin: boolean;
}

// Authentication store interface
interface AuthStore {
  user: User | null;
  loading: boolean;
  error: string | null;
  isAuthenticated: boolean;
}

// Initial state
const initialState: AuthStore = {
  user: null,
  loading: false,
  error: null,
  isAuthenticated: false,
};

// Create the writable store
const createAuthStore = () => {
  const { subscribe, set, update } = writable<AuthStore>(initialState);

  return {
    subscribe,
    
    // Initialize the store by checking for an existing token
    initialize: async () => {
      if (!isAuthenticated()) {
        return;
      }

      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        // Validate the token and get user info
        const authData = await api.auth.validateToken();
        
        // Get full user info
        const userInfo = await api.users.getCurrent();
        
        update(state => ({
          ...state,
          user: userInfo,
          isAuthenticated: true,
          loading: false,
          error: null,
        }));
      } catch (error) {
        // If token validation fails, remove it
        removeToken();
        update(state => ({
          ...state,
          user: null,
          isAuthenticated: false,
          loading: false,
          error: 'Session expired. Please log in again.',
        }));
      }
    },
    
    // Login method
    login: async (username: string, password: string) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const response = await api.auth.login(username, password);
        setToken(response.token);
        
        // Get full user info
        const userInfo = await api.users.getCurrent();
        
        update(state => ({
          ...state,
          user: userInfo,
          isAuthenticated: true,
          loading: false,
          error: null,
        }));
        
        return true;
      } catch (error) {
        let errorMessage = 'Login failed. Please check your credentials.';
        if (error instanceof Error) {
          errorMessage = error.message;
        }
        
        update(state => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        
        return false;
      }
    },
    
    // Register method
    register: async (username: string, email: string, password: string) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        await api.auth.register(username, email, password);
        
        // After registration, log the user in
        return await authStore.login(username, password);
      } catch (error) {
        let errorMessage = 'Registration failed. Please try again.';
        if (error instanceof Error) {
          errorMessage = error.message;
        }
        
        update(state => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        
        return false;
      }
    },
    
    // Logout method
    logout: () => {
      removeToken();
      set(initialState);
    },
    
    // Update user info
    updateUserInfo: async (data: Partial<User>) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const updatedUser = await api.users.update(data);
        
        update(state => ({
          ...state,
          user: updatedUser,
          loading: false,
          error: null,
        }));
        
        return true;
      } catch (error) {
        let errorMessage = 'Failed to update user info. Please try again.';
        if (error instanceof Error) {
          errorMessage = error.message;
        }
        
        update(state => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        
        return false;
      }
    },
    
    // Change password
    changePassword: async (current_password: string, new_password: string) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        await api.users.changePassword(current_password, new_password);
        
        update(state => ({
          ...state,
          loading: false,
          error: null,
        }));
        
        return true;
      } catch (error) {
        let errorMessage = 'Failed to change password. Please try again.';
        if (error instanceof Error) {
          errorMessage = error.message;
        }
        
        update(state => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        
        return false;
      }
    },
    
    // Clear any errors
    clearError: () => {
      update(state => ({ ...state, error: null }));
    },
  };
};

// Create and export the store
export const authStore = createAuthStore();

// Helper functions to update the store
export const authActions = {
  setUser: (user: User | null) => {
    authStore.update(state => ({ ...state, user, isAuthenticated: !!user }));
  },
  
  setLoading: (isLoading: boolean) => {
    authStore.update(state => ({ ...state, loading: isLoading }));
  },
  
  logout: () => {
    authStore.set({ ...initialState, loading: false });
  },
  
  initialize: async () => {
    // Set loading state
    authStore.update(state => ({ ...state, loading: true }));
    
    try {
      // Check if user has a valid session
      // This would be an API call to your backend to verify the token/session
      const response = await fetch('/api/auth/me');
      
      if (response.ok) {
        const user = await response.json();
        authStore.update(state => ({
          user,
          isAuthenticated: true,
          loading: false
        }));
      } else {
        // No valid session found
        authStore.update(state => ({
          user: null,
          isAuthenticated: false,
          loading: false
        }));
      }
    } catch (error) {
      console.error('Failed to initialize auth store:', error);
      // Reset auth state on error
      authStore.update(state => ({
        user: null,
        isAuthenticated: false,
        loading: false
      }));
    }
  }
}; 