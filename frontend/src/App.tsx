import React, { Suspense, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { CssBaseline, GlobalStyles } from '@mui/material';
import { QueryClient, QueryClientProvider } from 'react-query';
import { motion, AnimatePresence } from 'framer-motion';

import { AuthProvider, useAuth } from './hooks/useAuth';
import { WebSocketProvider } from './contexts/WebSocketContext';
import { AIAssistantProvider } from './contexts/AIAssistantContext';

// Components
import NavigationBar from './components/Navigation/NavigationBar';
import LoadingScreen from './components/UI/LoadingScreen';
import AIFloatingAssistant from './components/AI/AIFloatingAssistant';
import NotificationCenter from './components/Notifications/NotificationCenter';
import ErrorBoundary from './components/ErrorBoundary/ErrorBoundary';

// Lazy load components for optimal performance
const Dashboard = React.lazy(() => import('./pages/Dashboard'));
const MealPlanning = React.lazy(() => import('./pages/MealPlanning'));
const NutritionTracking = React.lazy(() => import('./pages/NutritionTracking'));
const RecipeManager = React.lazy(() => import('./pages/RecipeManager'));
const ShoppingLists = React.lazy(() => import('./pages/ShoppingLists'));
const Analytics = React.lazy(() => import('./pages/Analytics'));
const Profile = React.lazy(() => import('./pages/Profile'));
const Login = React.lazy(() => import('./pages/Login'));
const Register = React.lazy(() => import('./pages/Register'));

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes
      cacheTime: 10 * 60 * 1000, // 10 minutes
      refetchOnWindowFocus: false,
    },
  },
});

const theme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: '#00BCD4',
      light: '#4DD0E1',
      dark: '#00838F',
    },
    secondary: {
      main: '#FF6B35',
      light: '#FF8A65',
      dark: '#E64A19',
    },
    background: {
      default: '#FAFAFA',
      paper: '#FFFFFF',
    },
    success: {
      main: '#4CAF50',
    },
    warning: {
      main: '#FF9800',
    },
    error: {
      main: '#F44336',
    },
  },
  typography: {
    fontFamily: '"Inter", "Roboto", "Helvetica", "Arial", sans-serif',
    h1: {
      fontWeight: 700,
      fontSize: '2.5rem',
    },
    h2: {
      fontWeight: 600,
      fontSize: '2rem',
    },
    h3: {
      fontWeight: 600,
      fontSize: '1.75rem',
    },
    body1: {
      fontSize: '1rem',
      lineHeight: 1.6,
    },
  },
  shape: {
    borderRadius: 12,
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: 'none',
          fontWeight: 600,
          borderRadius: 8,
          boxShadow: 'none',
          '&:hover': {
            boxShadow: '0 4px 8px rgba(0,0,0,0.12)',
          },
        },
      },
    },
    MuiCard: {
      styleOverrides: {
        root: {
          boxShadow: '0 2px 8px rgba(0,0,0,0.08)',
          '&:hover': {
            boxShadow: '0 4px 16px rgba(0,0,0,0.12)',
          },
        },
      },
    },
  },
});

const globalStyles = (
  <GlobalStyles
    styles={{
      body: {
        margin: 0,
        padding: 0,
        fontFamily: theme.typography.fontFamily,
        backgroundColor: theme.palette.background.default,
      },
      '*': {
        boxSizing: 'border-box',
      },
      '::-webkit-scrollbar': {
        width: '8px',
      },
      '::-webkit-scrollbar-track': {
        backgroundColor: '#f1f1f1',
      },
      '::-webkit-scrollbar-thumb': {
        backgroundColor: '#c1c1c1',
        borderRadius: '4px',
      },
      '::-webkit-scrollbar-thumb:hover': {
        backgroundColor: '#a8a8a8',
      },
    }}
  />
);

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, loading } = useAuth();

  if (loading) {
    return <LoadingScreen />;
  }

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
}

function PublicRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, loading } = useAuth();

  if (loading) {
    return <LoadingScreen />;
  }

  if (isAuthenticated) {
    return <Navigate to="/dashboard" replace />;
  }

  return <>{children}</>;
}

const pageVariants = {
  initial: { opacity: 0, y: 20 },
  in: { opacity: 1, y: 0 },
  out: { opacity: 0, y: -20 },
};

const pageTransition = {
  type: 'tween',
  ease: 'anticipate',
  duration: 0.5,
};

function App() {
  useEffect(() => {
    // Progressive Web App registration
    if ('serviceWorker' in navigator) {
      navigator.serviceWorker.register('/sw.js');
    }
  }, []);

  return (
    <ErrorBoundary>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider theme={theme}>
          <CssBaseline />
          {globalStyles}
          <AuthProvider>
            <WebSocketProvider>
              <AIAssistantProvider>
                <Router>
                  <div className="App">
                    <AnimatePresence mode="wait">
                      <Routes>
                        <Route
                          path="/login"
                          element={
                            <PublicRoute>
                              <Suspense fallback={<LoadingScreen />}>
                                <motion.div
                                  initial="initial"
                                  animate="in"
                                  exit="out"
                                  variants={pageVariants}
                                  transition={pageTransition}
                                >
                                  <Login />
                                </motion.div>
                              </Suspense>
                            </PublicRoute>
                          }
                        />
                        <Route
                          path="/register"
                          element={
                            <PublicRoute>
                              <Suspense fallback={<LoadingScreen />}>
                                <motion.div
                                  initial="initial"
                                  animate="in"
                                  exit="out"
                                  variants={pageVariants}
                                  transition={pageTransition}
                                >
                                  <Register />
                                </motion.div>
                              </Suspense>
                            </PublicRoute>
                          }
                        />
                        <Route
                          path="/*"
                          element={
                            <ProtectedRoute>
                              <NavigationBar />
                              <main style={{ marginTop: '64px', minHeight: 'calc(100vh - 64px)' }}>
                                <Suspense fallback={<LoadingScreen />}>
                                  <AnimatePresence mode="wait">
                                    <Routes>
                                      <Route
                                        path="/dashboard"
                                        element={
                                          <motion.div
                                            initial="initial"
                                            animate="in"
                                            exit="out"
                                            variants={pageVariants}
                                            transition={pageTransition}
                                          >
                                            <Dashboard />
                                          </motion.div>
                                        }
                                      />
                                      <Route
                                        path="/meal-planning"
                                        element={
                                          <motion.div
                                            initial="initial"
                                            animate="in"
                                            exit="out"
                                            variants={pageVariants}
                                            transition={pageTransition}
                                          >
                                            <MealPlanning />
                                          </motion.div>
                                        }
                                      />
                                      <Route
                                        path="/nutrition"
                                        element={
                                          <motion.div
                                            initial="initial"
                                            animate="in"
                                            exit="out"
                                            variants={pageVariants}
                                            transition={pageTransition}
                                          >
                                            <NutritionTracking />
                                          </motion.div>
                                        }
                                      />
                                      <Route
                                        path="/recipes"
                                        element={
                                          <motion.div
                                            initial="initial"
                                            animate="in"
                                            exit="out"
                                            variants={pageVariants}
                                            transition={pageTransition}
                                          >
                                            <RecipeManager />
                                          </motion.div>
                                        }
                                      />
                                      <Route
                                        path="/shopping"
                                        element={
                                          <motion.div
                                            initial="initial"
                                            animate="in"
                                            exit="out"
                                            variants={pageVariants}
                                            transition={pageTransition}
                                          >
                                            <ShoppingLists />
                                          </motion.div>
                                        }
                                      />
                                      <Route
                                        path="/analytics"
                                        element={
                                          <motion.div
                                            initial="initial"
                                            animate="in"
                                            exit="out"
                                            variants={pageVariants}
                                            transition={pageTransition}
                                          >
                                            <Analytics />
                                          </motion.div>
                                        }
                                      />
                                      <Route
                                        path="/profile"
                                        element={
                                          <motion.div
                                            initial="initial"
                                            animate="in"
                                            exit="out"
                                            variants={pageVariants}
                                            transition={pageTransition}
                                          >
                                            <Profile />
                                          </motion.div>
                                        }
                                      />
                                      <Route path="/" element={<Navigate to="/dashboard" replace />} />
                                    </Routes>
                                  </AnimatePresence>
                                </Suspense>
                              </main>
                              <AIFloatingAssistant />
                              <NotificationCenter />
                            </ProtectedRoute>
                          }
                        />
                      </Routes>
                    </AnimatePresence>
                  </div>
                </Router>
              </AIAssistantProvider>
            </WebSocketProvider>
          </AuthProvider>
        </ThemeProvider>
      </QueryClientProvider>
    </ErrorBoundary>
  );
}

export default App;
