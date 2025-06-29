import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../store';
import { logout as logoutAction, loginStart, loginSuccess } from '../store/slices/authSlice';

interface User {
  id: string;
  username: string;
  email: string;
}

interface UseAuthReturn {
  user: RootState['auth']['user'];
  token: RootState['auth']['token'];
  isAuthenticated: boolean;
  loading: boolean;
  login: (token: string, user?: User) => void;
  logout: () => void;
}

export const useAuth = (): UseAuthReturn => {
  const dispatch = useDispatch();
  const auth = useSelector((state: RootState) => state.auth);

  const logout = () => {
    dispatch(logoutAction());
  };
  
  const login = (token: string, user?: User) => {
    dispatch(loginStart());
    const mockUser = user || {
      id: '1',
      username: 'user',
      email: 'user@example.com',
    };
    dispatch(loginSuccess({ user: mockUser, token }));
  };

  return {
    ...auth,
    login,
    logout,
  };
};
