import { useState, useEffect } from 'react'
import { useApp } from '../store'

interface SettingsWindowProps {
  onClose: () => void
}

interface AudioDevice {
  id: string
  name: string
}

export function SettingsWindow({ onClose }: SettingsWindowProps) {
  const { settings, updateSettings } = useApp()
  const [microphones, setMicrophones] = useState<AudioDevice[]>([])
  const [localApiKey, setLocalApiKey] = useState(settings.apiKey)
  const [showApiKey, setShowApiKey] = useState(false)

  useEffect(() => {
    // Get available microphones from Tauri backend
    const fetchMicrophones = async () => {
      if (window.__TAURI__) {
        try {
          // This would be a Tauri command to list audio devices
          // For now, we'll use a placeholder
          setMicrophones([
            { id: 'default', name: 'Default Microphone' },
          ])
        } catch (e) {
          console.error('Failed to fetch microphones:', e)
        }
      } else {
        setMicrophones([
          { id: 'default', name: 'Default Microphone' },
        ])
      }
    }
    fetchMicrophones()
  }, [])

  const handleSaveApiKey = () => {
    updateSettings({ apiKey: localApiKey })
    // In production, this would save to .env or secure storage
  }

  const handleLanguageChange = (lang: 'pl' | 'en') => {
    updateSettings({ language: lang })
  }

  const handleMicrophoneChange = (deviceId: string) => {
    updateSettings({ microphone: deviceId })
  }

  return (
    <div className="settings-window">
      <div className="settings-header">
        <h2>{settings.language === 'pl' ? 'Ustawienia' : 'Settings'}</h2>
        <button className="close-btn" onClick={onClose}>
          &times;
        </button>
      </div>

      <div className="settings-content">
        {/* Language */}
        <div className="settings-group">
          <label className="settings-label">
            {settings.language === 'pl' ? 'Język interfejsu' : 'Interface Language'}
          </label>
          <div className="language-buttons">
            <button
              className={`lang-btn ${settings.language === 'pl' ? 'active' : ''}`}
              onClick={() => handleLanguageChange('pl')}
            >
              Polski
            </button>
            <button
              className={`lang-btn ${settings.language === 'en' ? 'active' : ''}`}
              onClick={() => handleLanguageChange('en')}
            >
              English
            </button>
          </div>
        </div>

        {/* Microphone */}
        <div className="settings-group">
          <label className="settings-label">
            {settings.language === 'pl' ? 'Mikrofon' : 'Microphone'}
          </label>
          <select
            className="settings-select"
            value={settings.microphone}
            onChange={(e) => handleMicrophoneChange(e.target.value)}
          >
            {microphones.map((mic) => (
              <option key={mic.id} value={mic.id}>
                {mic.name}
              </option>
            ))}
          </select>
        </div>

        {/* API Key */}
        <div className="settings-group">
          <label className="settings-label">
            DeepSeek API Key
          </label>
          <div className="api-key-input">
            <input
              type={showApiKey ? 'text' : 'password'}
              value={localApiKey}
              onChange={(e) => setLocalApiKey(e.target.value)}
              placeholder="sk-..."
              className="settings-input"
            />
            <button
              className="toggle-visibility"
              onClick={() => setShowApiKey(!showApiKey)}
            >
              {showApiKey ? '🙈' : '👁️'}
            </button>
          </div>
          <button className="save-btn" onClick={handleSaveApiKey}>
            {settings.language === 'pl' ? 'Zapisz' : 'Save'}
          </button>
          <p className="settings-hint">
            {settings.language === 'pl'
              ? 'Klucz API jest wymagany do post-processingu tekstu'
              : 'API key is required for text post-processing'}
          </p>
        </div>

        {/* Hotkey */}
        <div className="settings-group">
          <label className="settings-label">
            {settings.language === 'pl' ? 'Skrót klawiszowy' : 'Keyboard Shortcut'}
          </label>
          <div className="hotkey-display">
            <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Space</kbd>
          </div>
          <p className="settings-hint">
            {settings.language === 'pl'
              ? 'Skrót można zmienić w przyszłych wersjach'
              : 'Shortcut can be changed in future versions'}
          </p>
        </div>
      </div>

      <div className="settings-footer">
        <span className="version">ValleyFlow v1.0.0</span>
      </div>
    </div>
  )
}
