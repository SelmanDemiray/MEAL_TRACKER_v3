import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../store';
import { logout as logoutAction, loginStart, loginSuccess } from '../store/slices/authSlice';

// Define an interface for the return type of the useAuth hook
interface UseAuthReturn {
  user: RootState['auth']['user'];
  token: RootState['auth']['token'];
  isAuthenticated: boolean;
  loading: boolean;
  login: (token: string) => void;
  logout: () => void;
}

export const useAuth = (): UseAuthReturn => {
  const dispatch = useDispatch();
  const auth = useSelector((state: RootState) => state.auth);

  const logout = () => {
    dispatch(logoutAction());
  };
  
  const login = (token: string) => {
    dispatch(loginStart());
    
    // Since this is a mock login, create a simple user object
    const mockUser = {
      id: '1',
      username: 'user',
      email: 'user@example.com',
    };
    
    // Dispatch login success with the mock user and token
    dispatch(loginSuccess({ user: mockUser, token }));
  };

  return {
    ...auth,
    login,
    logout,
  };
};
