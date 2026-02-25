import { useState, useEffect } from 'react'

interface TranscriptionEvent {
  type: string
  payload: string | boolean | number
}

declare global {
  interface Window {
    __TAURI__?: {
      event: {
        listen: <T>(
          event: string,
          handler: (event: { payload: T }) => void
        ) => Promise<() => void>
      }
    }
  }
}

type AppStatus =
  | 'idle'
  | 'recording'
  | 'processing'
  | 'success'
  | 'error'

function App() {
  const [status, setStatus] = useState<AppStatus>('idle')
  const [recordingTime, setRecordingTime] = useState(0)
  const [lastTranscription, setLastTranscription] = useState('')
  const [errorMessage, setErrorMessage] = useState('')
  const [language, setLanguage] = useState<'pl' | 'en'>('pl')

  useEffect(() => {
    if (!window.__TAURI__) return

    const unlisteners: (() => void)[] = []

    // Listen for recording state changes
    window.__TAURI__.event.listen<boolean>('recording-state', (event) => {
      if (event.payload) {
        setStatus('recording')
        setRecordingTime(0)
        setErrorMessage('')
      } else {
        setStatus('processing')
      }
    }).then((unlisten) => unlisteners.push(unlisten))

    // Listen for processing state
    window.__TAURI__.event.listen<boolean>('recording-processing', (event) => {
      if (!event.payload) {
        setStatus('success')
        setTimeout(() => setStatus('idle'), 2000)
      }
    }).then((unlisten) => unlisteners.push(unlisten))

    // Listen for raw transcription
    window.__TAURI__.event.listen<string>('transcription-raw', (event) => {
      console.log('Raw transcription:', event.payload)
    }).then((unlisten) => unlisteners.push(unlisten))

    // Listen for completed transcription
    window.__TAURI__.event.listen<string>('transcription-complete', (event) => {
      setLastTranscription(event.payload)
      console.log('Final transcription:', event.payload)
    }).then((unlisten) => unlisteners.push(unlisten))

    // Listen for errors
    window.__TAURI__.event.listen<string>('recording-error', (event) => {
      setStatus('error')
      setErrorMessage(event.payload)
      setTimeout(() => setStatus('idle'), 3000)
    }).then((unlisten) => unlisteners.push(unlisten))

    // Cleanup
    return () => {
      unlisteners.forEach((unlisten) => unlisten())
    }
  }, [])

  // Recording timer
  useEffect(() => {
    let interval: ReturnType<typeof setInterval>
    if (status === 'recording') {
      interval = setInterval(() => {
        setRecordingTime((t) => {
          // Auto-stop at 5 minutes (300 seconds)
          if (t >= 299) return t
          return t + 1
        })
      }, 1000)
    }
    return () => clearInterval(interval)
  }, [status])

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
  }

  const getStatusText = (): string => {
    switch (status) {
      case 'idle':
        return language === 'pl' ? 'Gotowy' : 'Ready'
      case 'recording':
        return language === 'pl' ? 'Nagrywanie...' : 'Recording...'
      case 'processing':
        return language === 'pl' ? 'Przetwarzanie...' : 'Processing...'
      case 'success':
        return language === 'pl' ? 'Skopiowano!' : 'Copied!'
      case 'error':
        return language === 'pl' ? 'Błąd' : 'Error'
    }
  }

  return (
    <div className="container">
      <h1>ValleyFlow</h1>

      <div className={`status-indicator ${status}`}>
        <span className="status-text">{getStatusText()}</span>
        {status === 'recording' && (
          <span className="timer">{formatTime(recordingTime)}</span>
        )}
      </div>

      {status === 'recording' && (
        <p className="hint">
          {language === 'pl'
            ? 'Naciśnij Ctrl+Shift+Space aby zakończyć'
            : 'Press Ctrl+Shift+Space to stop'}
        </p>
      )}

      {status === 'idle' && (
        <p className="hint">
          {language === 'pl'
            ? 'Naciśnij Ctrl+Shift+Space aby rozpocząć nagrywanie'
            : 'Press Ctrl+Shift+Space to start recording'}
        </p>
      )}

      {status === 'processing' && (
        <div className="processing">
          <div className="spinner"></div>
        </div>
      )}

      {status === 'error' && (
        <p className="error">{errorMessage}</p>
      )}

      {lastTranscription && status === 'idle' && (
        <div className="last-transcription">
          <p className="label">
            {language === 'pl' ? 'Ostatnia transkrypcja:' : 'Last transcription:'}
          </p>
          <p className="text">{lastTranscription}</p>
        </div>
      )}

      <div className="language-toggle">
        <button
          className={language === 'pl' ? 'active' : ''}
          onClick={() => setLanguage('pl')}
        >
          PL
        </button>
        <button
          className={language === 'en' ? 'active' : ''}
          onClick={() => setLanguage('en')}
        >
          EN
        </button>
      </div>
    </div>
  )
}

export default App
