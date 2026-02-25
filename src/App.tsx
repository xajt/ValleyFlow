import { useState, useEffect } from 'react'
import { AppProvider, useApp } from './store'
import {
  RecordingOverlay,
  SettingsWindow,
  HistoryWindow,
  WelcomeWizard,
} from './components'

function AppContent() {
  const {
    settings,
    isRecording,
    isProcessing,
    recordingTime,
    setRecording,
    setProcessing,
    setRecordingTime,
    addToHistory,
  } = useApp()

  const [showSettings, setShowSettings] = useState(false)
  const [showHistory, setShowHistory] = useState(false)
  const [showWizard, setShowWizard] = useState(!settings.hasCompletedWizard)

  // Listen for Tauri events
  useEffect(() => {
    if (!window.__TAURI__) return

    const unlisteners: (() => void)[] = []

    window.__TAURI__.event.listen<boolean>('recording-state', (event) => {
      setRecording(event.payload)
      if (event.payload) {
        setRecordingTime(0)
      }
    }).then((unlisten) => unlisteners.push(unlisten))

    window.__TAURI__.event.listen<boolean>('recording-processing', (event) => {
      setProcessing(event.payload)
    }).then((unlisten) => unlisteners.push(unlisten))

    window.__TAURI__.event.listen<string>('transcription-complete', (event) => {
      addToHistory({
        text: event.payload,
        rawText: event.payload, // TODO: get raw text separately
        language: settings.language,
      })
    }).then((unlisten) => unlisteners.push(unlisten))

    return () => {
      unlisteners.forEach((unlisten) => unlisten())
    }
  }, [setRecording, setProcessing, setRecordingTime, addToHistory, settings.language])

  // Recording timer
  useEffect(() => {
    let interval: ReturnType<typeof setInterval>
    if (isRecording) {
      interval = setInterval(() => {
        setRecordingTime((t) => (t >= 299 ? t : t + 1))
      }, 1000)
    }
    return () => clearInterval(interval)
  }, [isRecording, setRecordingTime])

  // Listen for tray menu events
  useEffect(() => {
    if (!window.__TAURI__) return

    const unlisteners: (() => void)[] = []

    window.__TAURI__.event.listen('open-settings', () => {
      setShowSettings(true)
    }).then((unlisten) => unlisteners.push(unlisten))

    window.__TAURI__.event.listen('open-history', () => {
      setShowHistory(true)
    }).then((unlisten) => unlisteners.push(unlisten))

    return () => {
      unlisteners.forEach((unlisten) => unlisten())
    }
  }, [])

  const handleCancelRecording = () => {
    setRecording(false)
    setRecordingTime(0)
    // In production, this would notify the Rust backend
  }

  const handleWizardComplete = () => {
    setShowWizard(false)
  }

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
  }

  // Show wizard on first run
  if (showWizard) {
    return <WelcomeWizard onComplete={handleWizardComplete} />
  }

  return (
    <div className="app">
      {/* Main content (hidden when in tray mode) */}
      <div className="main-content">
        <h1>ValleyFlow</h1>

        <div className={`status-indicator ${isRecording ? 'recording' : isProcessing ? 'processing' : 'idle'}`}>
          {isRecording && (
            <>
              <span className="recording-dot"></span>
              <span>{settings.language === 'pl' ? 'Nagrywanie' : 'Recording'}...</span>
              <span className="timer">{formatTime(recordingTime)}</span>
            </>
          )}
          {isProcessing && (
            <>
              <div className="spinner-small"></div>
              <span>{settings.language === 'pl' ? 'Przetwarzanie' : 'Processing'}...</span>
            </>
          )}
          {!isRecording && !isProcessing && (
            <span>{settings.language === 'pl' ? 'Gotowy' : 'Ready'}</span>
          )}
        </div>

        <p className="hint">
          {settings.language === 'pl'
            ? 'Naciśnij Ctrl+Shift+Space aby rozpocząć nagrywanie'
            : 'Press Ctrl+Shift+Space to start recording'}
        </p>

        <div className="quick-actions">
          <button onClick={() => setShowSettings(true)}>
            {settings.language === 'pl' ? 'Ustawienia' : 'Settings'}
          </button>
          <button onClick={() => setShowHistory(true)}>
            {settings.language === 'pl' ? 'Historia' : 'History'}
          </button>
        </div>
      </div>

      {/* Recording Overlay */}
      <RecordingOverlay onCancel={handleCancelRecording} />

      {/* Settings Modal */}
      {showSettings && (
        <div className="modal-overlay" onClick={() => setShowSettings(false)}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <SettingsWindow onClose={() => setShowSettings(false)} />
          </div>
        </div>
      )}

      {/* History Modal */}
      {showHistory && (
        <div className="modal-overlay" onClick={() => setShowHistory(false)}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <HistoryWindow onClose={() => setShowHistory(false)} />
          </div>
        </div>
      )}
    </div>
  )
}

function App() {
  return (
    <AppProvider>
      <AppContent />
    </AppProvider>
  )
}

export default App
