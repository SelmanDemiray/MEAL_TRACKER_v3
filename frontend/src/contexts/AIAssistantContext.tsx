import React, { createContext, useContext, useState, ReactNode } from 'react';

interface AIAssistantContextType {
  isOpen: boolean;
  toggleAssistant: () => void;
  askQuestion: (question: string) => Promise<string>;
  suggestions: string[];
}

const AIAssistantContext = createContext<AIAssistantContextType | undefined>(undefined);

export const useAIAssistant = () => {
  const context = useContext(AIAssistantContext);
  if (!context) {
    throw new Error('useAIAssistant must be used within an AIAssistantProvider');
  }
  return context;
};

interface AIAssistantProviderProps {
  children: ReactNode;
}

export const AIAssistantProvider: React.FC<AIAssistantProviderProps> = ({ children }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [suggestions] = useState([
    'What should I cook today?',
    'Help me plan meals for this week',
    'Suggest healthy breakfast options',
    'What can I make with chicken and vegetables?',
  ]);

  const toggleAssistant = () => {
    setIsOpen(!isOpen);
  };

  const askQuestion = async (question: string): Promise<string> => {
    // Simulate AI response - in production, this would call the AI service
    await new Promise((resolve) => setTimeout(resolve, 1000));

    return `I understand you're asking about: "${question}". Based on your preferences and nutrition goals, here are some suggestions...`;
  };

  return (
    <AIAssistantContext.Provider value={{ isOpen, toggleAssistant, askQuestion, suggestions }}>
      {children}
    </AIAssistantContext.Provider>
  );
};
