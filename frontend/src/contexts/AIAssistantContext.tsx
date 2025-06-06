import React, { createContext, useContext, useState, ReactNode } from 'react';

interface AIAssistantContextType {
  isVisible: boolean;
  showAssistant: () => void;
  hideAssistant: () => void;
  askQuestion: (question: string) => Promise<string>;
}

const AIAssistantContext = createContext<AIAssistantContextType | undefined>(undefined);

export const AIAssistantProvider = ({ children }: { children: ReactNode }) => {
  const [isVisible, setIsVisible] = useState(false);

  const showAssistant = () => setIsVisible(true);
  const hideAssistant = () => setIsVisible(false);

  const askQuestion = async (question: string): Promise<string> => {
    // Mock AI response - replace with actual AI service call
    await new Promise(resolve => setTimeout(resolve, 1000));
    return `Here's a helpful response to: "${question}". This is a mock AI assistant response.`;
  };

  return (
    <AIAssistantContext.Provider value={{
      isVisible,
      showAssistant,
      hideAssistant,
      askQuestion
    }}>
      {children}
    </AIAssistantContext.Provider>
  );
};

export const useAIAssistant = () => {
  const context = useContext(AIAssistantContext);
  if (context === undefined) {
    throw new Error('useAIAssistant must be used within an AIAssistantProvider');
  }
  return context;
};
