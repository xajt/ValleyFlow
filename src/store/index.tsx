import { createContext, useContext, useState, useEffect, ReactNode } from 'react'

export interface TranscriptionItem {
  id: string
  text: string
  rawText: string
  language: 'pl' | 'en'
  timestamp: number
}

export interface Settings {
  language: 'pl' | 'en'
  microphone: string
  apiKey: string
  hotkey: string
  hasCompletedWizard: boolean
}

interface AppState {
  settings: Settings
  history: TranscriptionItem[]
  isRecording: boolean
  isProcessing: boolean
  recordingTime: number
  updateSettings: (settings: Partial<Settings>) => void
  addToHistory: (item: Omit<TranscriptionItem, 'id' | 'timestamp'>) => void
  clearHistory: () => void
  setRecording: (isRecording: boolean) => void
  setProcessing: (isProcessing: boolean) => void
  setRecordingTime: (time: number) => void
}

const defaultSettings: Settings = {
  language: 'pl',
  microphone: 'default',
  apiKey: '',
  hotkey: 'Ctrl+Shift+Space',
  hasCompletedWizard: false,
}

const AppContext = createContext<AppState | undefined>(undefined)

export function AppProvider({ children }: { children: ReactNode }) {
  const [settings, setSettings] = useState<Settings>(() => {
    const saved = localStorage.getItem('valleyflow-settings')
    return saved ? { ...defaultSettings, ...JSON.parse(saved) } : defaultSettings
  })

  const [history, setHistory] = useState<TranscriptionItem[]>(() => {
    const saved = localStorage.getItem('valleyflow-history')
    return saved ? JSON.parse(saved) : []
  })

  const [isRecording, setIsRecording] = useState(false)
  const [isProcessing, setIsProcessing] = useState(false)
  const [recordingTime, setRecordingTime] = useState(0)

  useEffect(() => {
    localStorage.setItem('valleyflow-settings', JSON.stringify(settings))
  }, [settings])

  useEffect(() => {
    localStorage.setItem('valleyflow-history', JSON.stringify(history))
  }, [history])

  const updateSettings = (newSettings: Partial<Settings>) => {
    setSettings((prev) => ({ ...prev, ...newSettings }))
  }

  const addToHistory = (item: Omit<TranscriptionItem, 'id' | 'timestamp'>) => {
    const newItem: TranscriptionItem = {
      ...item,
      id: crypto.randomUUID(),
      timestamp: Date.now(),
    }
    setHistory((prev) => {
      const updated = [newItem, ...prev]
      return updated.slice(0, 50) // Keep max 50 items
    })
  }

  const clearHistory = () => {
    setHistory([])
  }

  const value: AppState = {
    settings,
    history,
    isRecording,
    isProcessing,
    recordingTime,
    updateSettings,
    addToHistory,
    clearHistory,
    setRecording: setIsRecording,
    setProcessing: setIsProcessing,
    setRecordingTime,
  }

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>
}

export function useApp() {
  const context = useContext(AppContext)
  if (!context) {
    throw new Error('useApp must be used within AppProvider')
  }
  return context
}
