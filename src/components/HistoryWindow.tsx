import { useState } from 'react'
import { useApp, TranscriptionItem } from '../store'

interface HistoryWindowProps {
  onClose: () => void
}

function formatDate(timestamp: number, language: 'pl' | 'en'): string {
  const date = new Date(timestamp)
  const locale = language === 'pl' ? 'pl-PL' : 'en-US'
  return date.toLocaleDateString(locale, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function HistoryCard({
  item,
  language,
  onCopy,
}: {
  item: TranscriptionItem
  language: 'pl' | 'en'
  onCopy: (text: string) => void
}) {
  const [isExpanded, setIsExpanded] = useState(false)
  const [copied, setCopied] = useState(false)

  const handleCopy = () => {
    onCopy(item.text)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const preview = item.text.length > 50 ? item.text.slice(0, 50) + '...' : item.text

  return (
    <div className="history-card" onClick={() => setIsExpanded(!isExpanded)}>
      <div className="card-header">
        <span className="card-date">{formatDate(item.timestamp, language)}</span>
        <span className={`card-lang ${item.language}`}>{item.language.toUpperCase()}</span>
      </div>

      {!isExpanded ? (
        <p className="card-preview">{preview}</p>
      ) : (
        <div className="card-expanded">
          <p className="card-full-text">{item.text}</p>
          <div className="card-actions">
            <button
              className="copy-btn"
              onClick={(e) => {
                e.stopPropagation()
                handleCopy()
              }}
            >
              {copied
                ? language === 'pl'
                  ? 'Skopiowano!'
                  : 'Copied!'
                : language === 'pl'
                  ? 'Kopiuj'
                  : 'Copy'}
            </button>
          </div>
        </div>
      )}

      <span className="expand-hint">
        {isExpanded
          ? language === 'pl'
            ? 'Kliknij aby zwinąć'
            : 'Click to collapse'
          : language === 'pl'
            ? 'Kliknij aby rozwinąć'
            : 'Click to expand'}
      </span>
    </div>
  )
}

export function HistoryWindow({ onClose }: HistoryWindowProps) {
  const { history, settings, clearHistory } = useApp()

  const handleCopy = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text)
    } catch (e) {
      console.error('Failed to copy:', e)
    }
  }

  return (
    <div className="history-window">
      <div className="history-header">
        <h2>{settings.language === 'pl' ? 'Historia' : 'History'}</h2>
        <div className="header-actions">
          {history.length > 0 && (
            <button className="clear-btn" onClick={clearHistory}>
              {settings.language === 'pl' ? 'Wyczyść' : 'Clear'}
            </button>
          )}
          <button className="close-btn" onClick={onClose}>
            &times;
          </button>
        </div>
      </div>

      <div className="history-content">
        {history.length === 0 ? (
          <div className="empty-state">
            <p>
              {settings.language === 'pl'
                ? 'Brak transkrypcji w historii'
                : 'No transcriptions in history'}
            </p>
            <p className="hint">
              {settings.language === 'pl'
                ? 'Naciśnij Ctrl+Shift+Space aby rozpocząć'
                : 'Press Ctrl+Shift+Space to start'}
            </p>
          </div>
        ) : (
          <div className="history-list">
            {history.map((item) => (
              <HistoryCard
                key={item.id}
                item={item}
                language={settings.language}
                onCopy={handleCopy}
              />
            ))}
          </div>
        )}
      </div>

      <div className="history-footer">
        <span className="count">
          {history.length}/50 {settings.language === 'pl' ? 'elementów' : 'items'}
        </span>
      </div>
    </div>
  )
}
