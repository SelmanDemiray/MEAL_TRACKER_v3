import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message: string;
  duration?: number;
  actions?: Array<{
    label: string;
    action: () => void;
  }>;
}

interface UIState {
  theme: 'light' | 'dark';
  sidebarOpen: boolean;
  loading: boolean;
  notifications: Notification[];
  modalOpen: boolean;
  modalComponent: string | null;
  modalProps: any;
}

const initialState: UIState = {
  theme: (localStorage.getItem('theme') as 'light' | 'dark') || 'light',
  sidebarOpen: true,
  loading: false,
  notifications: [],
  modalOpen: false,
  modalComponent: null,
  modalProps: null,
};

export const uiSlice = createSlice({
  name: 'ui',
  initialState,
  reducers: {
    toggleTheme: (state) => {
      state.theme = state.theme === 'light' ? 'dark' : 'light';
      localStorage.setItem('theme', state.theme);
    },
    setTheme: (state, action: PayloadAction<'light' | 'dark'>) => {
      state.theme = action.payload;
      localStorage.setItem('theme', state.theme);
    },
    toggleSidebar: (state) => {
      state.sidebarOpen = !state.sidebarOpen;
    },
    setSidebarOpen: (state, action: PayloadAction<boolean>) => {
      state.sidebarOpen = action.payload;
    },
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload;
    },
    addNotification: (state, action: PayloadAction<Omit<Notification, 'id'>>) => {
      const notification: Notification = {
        ...action.payload,
        id: Date.now().toString() + Math.random().toString(36).substr(2, 9),
      };
      state.notifications.push(notification);
    },
    removeNotification: (state, action: PayloadAction<string>) => {
      state.notifications = state.notifications.filter(
        (notification) => notification.id !== action.payload
      );
    },
    clearAllNotifications: (state) => {
      state.notifications = [];
    },
    openModal: (state, action: PayloadAction<{ component: string; props?: any }>) => {
      state.modalOpen = true;
      state.modalComponent = action.payload.component;
      state.modalProps = action.payload.props || null;
    },
    closeModal: (state) => {
      state.modalOpen = false;
      state.modalComponent = null;
      state.modalProps = null;
    },
  },
});

export const {
  toggleTheme,
  setTheme,
  toggleSidebar,
  setSidebarOpen,
  setLoading,
  addNotification,
  removeNotification,
  clearAllNotifications,
  openModal,
  closeModal,
} = uiSlice.actions;

export default uiSlice.reducer;
