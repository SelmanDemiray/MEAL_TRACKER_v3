import React, { createContext, useContext, useEffect, useState, ReactNode } from 'react';

interface WebSocketContextType {
  isConnected: boolean;
  sendMessage: (message: any) => void;
  lastMessage: any;
}

const WebSocketContext = createContext<WebSocketContextType | undefined>(undefined);

export const WebSocketProvider = ({ children }: { children: ReactNode }) => {
  const [isConnected, setIsConnected] = useState(false);
  const [lastMessage, setLastMessage] = useState<any>(null);
  const [ws, setWs] = useState<WebSocket | null>(null);

  useEffect(() => {
    // Mock WebSocket connection for development
    setIsConnected(true);
    
    // In production, replace with actual WebSocket connection:
    // const websocket = new WebSocket('ws://localhost:8080/ws');
    // websocket.onopen = () => setIsConnected(true);
    // websocket.onclose = () => setIsConnected(false);
    // websocket.onmessage = (event) => setLastMessage(JSON.parse(event.data));
    // setWs(websocket);
    
    return () => {
      if (ws) {
        ws.close();
      }
    };
  }, []);

  const sendMessage = (message: any) => {
    if (ws && isConnected) {
      ws.send(JSON.stringify(message));
    } else {
      console.log('Mock WebSocket message:', message);
    }
  };

  return (
    <WebSocketContext.Provider value={{
      isConnected,
      sendMessage,
      lastMessage
    }}>
      {children}
    </WebSocketContext.Provider>
  );
};

export const useWebSocket = () => {
  const context = useContext(WebSocketContext);
  if (context === undefined) {
    throw new Error('useWebSocket must be used within a WebSocketProvider');
  }
  return context;
};
