import { useState, useEffect } from 'react'

function App() {
  const [isRecording, setIsRecording] = useState(false)
  const [status, setStatus] = useState('Ready')

  useEffect(() => {
    // Listen for recording events from Rust backend
    const unlisten = window.__TAURI__?.event?.listen('recording-state', (event: unknown) => {
      const payload = (event as { payload: boolean }).payload
      setIsRecording(payload)
      setStatus(payload ? 'Recording...' : 'Ready')
    })

    return () => {
      unlisten?.then((fn: () => void) => fn())
    }
  }, [])

  return (
    <div className="container">
      <h1>ValleyFlow</h1>
      <p className="status">{status}</p>
      <p className="hint">Press Ctrl+Shift+Space to start/stop recording</p>
    </div>
  )
}

export default App
