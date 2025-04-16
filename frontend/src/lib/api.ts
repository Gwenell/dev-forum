import { API_URL, TOKEN_KEY } from './config';

// Type for HTTP methods
type Method = 'GET' | 'POST' | 'PUT' | 'DELETE';

// Interface for request options
interface RequestOptions {
  method?: Method;
  headers?: Record<string, string>;
  body?: any;
  withAuth?: boolean;
}

// Get authentication token from localStorage
const getToken = (): string | null => {
  if (typeof window !== 'undefined') {
    return localStorage.getItem(TOKEN_KEY);
  }
  return null;
};

// Set authentication token in localStorage
export const setToken = (token: string): void => {
  if (typeof window !== 'undefined') {
    localStorage.setItem(TOKEN_KEY, token);
  }
};

// Remove authentication token from localStorage
export const removeToken = (): void => {
  if (typeof window !== 'undefined') {
    localStorage.removeItem(TOKEN_KEY);
  }
};

// Check if user is authenticated
export const isAuthenticated = (): boolean => {
  return getToken() !== null;
};

// Make API request with appropriate headers and authentication
const request = async <T>(endpoint: string, options: RequestOptions = {}): Promise<T> => {
  const {
    method = 'GET',
    headers = {},
    body,
    withAuth = true,
  } = options;

  // Prepare headers
  const requestHeaders: Record<string, string> = {
    'Content-Type': 'application/json',
    ...headers,
  };

  // Add authentication token if required
  if (withAuth) {
    const token = getToken();
    if (token) {
      requestHeaders['Authorization'] = `Bearer ${token}`;
    }
  }

  // Prepare fetch options
  const fetchOptions: RequestInit = {
    method,
    headers: requestHeaders,
  };

  // Add body for non-GET requests
  if (body && method !== 'GET') {
    fetchOptions.body = JSON.stringify(body);
  }

  try {
    // Make the request
    const response = await fetch(`${API_URL}${endpoint}`, fetchOptions);
    
    // Handle non-successful responses
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      throw new Error(errorData.message || `API request failed with status ${response.status}`);
    }
    
    // Parse JSON response or return empty object if no content
    const data = await response.json().catch(() => ({}));
    return data as T;
  } catch (error) {
    console.error('API request failed:', error);
    throw error;
  }
};

// API endpoints
export const api = {
  // Auth endpoints
  auth: {
    login: (username: string, password: string) => 
      request<{token: string, user_id: string, username: string, is_admin: boolean}>(
        '/auth/login', 
        { method: 'POST', body: { username, password }, withAuth: false }
      ),
    register: (username: string, email: string, password: string) => 
      request<{user_id: string, username: string}>(
        '/auth/register', 
        { method: 'POST', body: { username, email, password }, withAuth: false }
      ),
    validateToken: () => 
      request<{user_id: string, username: string, is_admin: boolean}>(
        '/auth/validate'
      ),
  },

  // User endpoints
  users: {
    getCurrent: () => 
      request<any>('/users/me'),
    update: (data: any) => 
      request<any>('/users/me', { method: 'PUT', body: data }),
    changePassword: (current_password: string, new_password: string) => 
      request<any>('/users/change-password', { method: 'POST', body: { current_password, new_password } }),
    getById: (id: string) => 
      request<any>(`/users/${id}`),
  },

  // Categories endpoints
  categories: {
    getAll: () => 
      request<any[]>('/categories'),
    getById: (id: string) => 
      request<any>(`/categories/${id}`),
    getBySlug: (slug: string) => 
      request<any>(`/categories/slug/${slug}`),
    create: (data: any) => 
      request<any>('/categories/admin', { method: 'POST', body: data }),
    update: (id: string, data: any) => 
      request<any>(`/categories/admin/${id}`, { method: 'PUT', body: data }),
    delete: (id: string) => 
      request<void>(`/categories/admin/${id}`, { method: 'DELETE' }),
    createSubcategory: (categoryId: string, data: any) => 
      request<any>(`/categories/admin/${categoryId}/subcategories`, { method: 'POST', body: data }),
    updateSubcategory: (categoryId: string, subcategoryId: string, data: any) => 
      request<any>(`/categories/admin/${categoryId}/subcategories/${subcategoryId}`, { method: 'PUT', body: data }),
    deleteSubcategory: (categoryId: string, subcategoryId: string) => 
      request<void>(`/categories/admin/${categoryId}/subcategories/${subcategoryId}`, { method: 'DELETE' }),
  },

  // Threads endpoints
  threads: {
    getBySubcategory: (subcategoryId: string) => 
      request<any[]>(`/threads/subcategory/${subcategoryId}`),
    getById: (id: string) => 
      request<any>(`/threads/${id}`),
    create: (data: any) => 
      request<any>('/threads', { method: 'POST', body: data }),
    update: (id: string, data: any) => 
      request<any>(`/threads/${id}`, { method: 'PUT', body: data }),
    delete: (id: string) => 
      request<void>(`/threads/${id}`, { method: 'DELETE' }),
  },

  // Posts endpoints
  posts: {
    getByThread: (threadId: string) => 
      request<any[]>(`/posts/thread/${threadId}`),
    create: (data: any) => 
      request<any>('/posts', { method: 'POST', body: data }),
    update: (id: string, data: any) => 
      request<any>(`/posts/${id}`, { method: 'PUT', body: data }),
    delete: (id: string) => 
      request<void>(`/posts/${id}`, { method: 'DELETE' }),
  },

  // Files endpoints
  files: {
    getAll: () => 
      request<any[]>('/files'),
    getById: (id: string) => 
      request<any>(`/files/${id}`),
    // For file uploads, we'll need to use FormData instead of JSON
    upload: (formData: FormData) => {
      return request<any>('/files', { 
        method: 'POST', 
        body: formData,
        headers: { } // Remove Content-Type to let the browser set it with the boundary
      });
    },
    delete: (id: string) => 
      request<void>(`/files/${id}`, { method: 'DELETE' }),
  },

  // Streams endpoints
  streams: {
    getAll: () => 
      request<any[]>('/streams'),
    getActive: () => 
      request<any[]>('/streams/active'),
    getById: (id: string) => 
      request<any>(`/streams/${id}`),
    create: (data: any) => 
      request<any>('/streams', { method: 'POST', body: data }),
    update: (id: string, data: any) => 
      request<any>(`/streams/${id}`, { method: 'PUT', body: data }),
    delete: (id: string) => 
      request<void>(`/streams/${id}`, { method: 'DELETE' }),
    start: (id: string) => 
      request<any>(`/streams/${id}/start`, { method: 'POST' }),
    end: (id: string) => 
      request<any>(`/streams/${id}/end`, { method: 'POST' }),
  },
}; 