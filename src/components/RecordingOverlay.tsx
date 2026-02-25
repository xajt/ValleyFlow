import { useApp } from '../store'
import { useEffect, useState } from 'react'

interface RecordingOverlayProps {
  onCancel: () => void
}

export function RecordingOverlay({ onCancel }: RecordingOverlayProps) {
  const { isRecording, recordingTime, settings } = useApp()
  const [isVisible, setIsVisible] = useState(false)

  useEffect(() => {
    if (isRecording) {
      setIsVisible(true)
    } else {
      const timer = setTimeout(() => setIsVisible(false), 300)
      return () => clearTimeout(timer)
    }
  }, [isRecording])

  if (!isVisible) return null

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
  }

  const handleCancel = () => {
    onCancel()
  }

  return (
    <div className={`overlay ${isRecording ? 'visible' : 'hiding'}`}>
      <div className="overlay-content">
        <div className="recording-indicator">
          <span className="recording-dot"></span>
          <span className="recording-text">
            {settings.language === 'pl' ? 'Nagrywanie' : 'Recording'}...
          </span>
        </div>
        <span className="timer">{formatTime(recordingTime)}</span>
        <button className="cancel-btn" onClick={handleCancel}>
          {settings.language === 'pl' ? 'Anuluj' : 'Cancel'}
        </button>
      </div>
    </div>
  )
}
