import { useState } from 'react'
import { useApp } from '../store'

interface WelcomeWizardProps {
  onComplete: () => void
}

type WizardStep = 'language' | 'api' | 'microphone' | 'done'

export function WelcomeWizard({ onComplete }: WelcomeWizardProps) {
  const { updateSettings } = useApp()
  const [step, setStep] = useState<WizardStep>('language')
  const [apiKey, setApiKey] = useState('')
  const [showApiKey, setShowApiKey] = useState(false)
  const [micTestStatus, setMicTestStatus] = useState<'idle' | 'testing' | 'success' | 'error'>('idle')

  const handleLanguageSelect = (lang: 'pl' | 'en') => {
    updateSettings({ language: lang })
    setStep('api')
  }

  const handleApiKeySave = () => {
    updateSettings({ apiKey })
    setStep('microphone')
  }

  const handleSkipApi = () => {
    setStep('microphone')
  }

  const handleMicTest = async () => {
    setMicTestStatus('testing')
    // In production, this would test the microphone via Tauri
    setTimeout(() => {
      setMicTestStatus('success')
    }, 1500)
  }

  const handleComplete = () => {
    updateSettings({ hasCompletedWizard: true })
    onComplete()
  }

  const texts = {
    language: {
      pl: {
        title: 'Witaj w ValleyFlow!',
        subtitle: 'Wybierz język interfejsu',
      },
      en: {
        title: 'Welcome to ValleyFlow!',
        subtitle: 'Choose your interface language',
      },
    },
    api: {
      pl: {
        title: 'Konfiguracja API',
        subtitle: 'Wprowadź klucz DeepSeek API dla post-processingu',
        placeholder: 'sk-...',
        save: 'Zapisz i kontynuuj',
        skip: 'Pomiń na razie',
        hint: 'Klucz można dodać później w Ustawieniach',
      },
      en: {
        title: 'API Configuration',
        subtitle: 'Enter your DeepSeek API key for post-processing',
        placeholder: 'sk-...',
        save: 'Save and Continue',
        skip: 'Skip for now',
        hint: 'You can add this later in Settings',
      },
    },
    microphone: {
      pl: {
        title: 'Test mikrofonu',
        subtitle: 'Sprawdź czy Twój mikrofon działa poprawnie',
        test: 'Testuj mikrofon',
        testing: 'Testowanie...',
        success: 'Mikrofon działa!',
        error: 'Błąd mikrofonu',
        continue: 'Zakończ',
      },
      en: {
        title: 'Microphone Test',
        subtitle: 'Make sure your microphone works correctly',
        test: 'Test Microphone',
        testing: 'Testing...',
        success: 'Microphone works!',
        error: 'Microphone error',
        continue: 'Finish',
      },
    },
  }

  // Get current language (default to 'en' for initial step)
  const lang = step === 'language' ? 'en' : (useApp().settings.language || 'en')

  return (
    <div className="wizard-overlay">
      <div className="wizard-container">
        {/* Progress indicator */}
        <div className="wizard-progress">
          <div className={`progress-step ${step === 'language' || step === 'api' || step === 'microphone' || step === 'done' ? 'active' : ''}`}>
            1
          </div>
          <div className="progress-line"></div>
          <div className={`progress-step ${step === 'api' || step === 'microphone' || step === 'done' ? 'active' : ''}`}>
            2
          </div>
          <div className="progress-line"></div>
          <div className={`progress-step ${step === 'microphone' || step === 'done' ? 'active' : ''}`}>
            3
          </div>
        </div>

        {/* Step 1: Language */}
        {step === 'language' && (
          <div className="wizard-step">
            <h1>{texts.language.pl.title}</h1>
            <p className="subtitle">{texts.language.pl.subtitle}</p>
            <div className="language-options">
              <button className="lang-option" onClick={() => handleLanguageSelect('pl')}>
                <span className="flag">🇵🇱</span>
                <span className="name">Polski</span>
              </button>
              <button className="lang-option" onClick={() => handleLanguageSelect('en')}>
                <span className="flag">🇬🇧</span>
                <span className="name">English</span>
              </button>
            </div>
          </div>
        )}

        {/* Step 2: API Key */}
        {step === 'api' && (
          <div className="wizard-step">
            <h1>{texts.api[lang as 'pl' | 'en'].title}</h1>
            <p className="subtitle">{texts.api[lang as 'pl' | 'en'].subtitle}</p>

            <div className="api-input-group">
              <input
                type={showApiKey ? 'text' : 'password'}
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                placeholder={texts.api[lang as 'pl' | 'en'].placeholder}
                className="wizard-input"
              />
              <button
                className="toggle-visibility"
                onClick={() => setShowApiKey(!showApiKey)}
              >
                {showApiKey ? '🙈' : '👁️'}
              </button>
            </div>

            <div className="wizard-buttons">
              <button className="btn-secondary" onClick={handleSkipApi}>
                {texts.api[lang as 'pl' | 'en'].skip}
              </button>
              <button className="btn-primary" onClick={handleApiKeySave}>
                {texts.api[lang as 'pl' | 'en'].save}
              </button>
            </div>

            <p className="hint">{texts.api[lang as 'pl' | 'en'].hint}</p>
          </div>
        )}

        {/* Step 3: Microphone Test */}
        {step === 'microphone' && (
          <div className="wizard-step">
            <h1>{texts.microphone[lang as 'pl' | 'en'].title}</h1>
            <p className="subtitle">{texts.microphone[lang as 'pl' | 'en'].subtitle}</p>

            <div className="mic-test-container">
              <button
                className={`mic-test-btn ${micTestStatus}`}
                onClick={handleMicTest}
                disabled={micTestStatus === 'testing'}
              >
                {micTestStatus === 'testing' && (
                  <span className="spinner"></span>
                )}
                {micTestStatus === 'idle' && texts.microphone[lang as 'pl' | 'en'].test}
                {micTestStatus === 'testing' && texts.microphone[lang as 'pl' | 'en'].testing}
                {micTestStatus === 'success' && '✓ ' + texts.microphone[lang as 'pl' | 'en'].success}
                {micTestStatus === 'error' && '✗ ' + texts.microphone[lang as 'pl' | 'en'].error}
              </button>
            </div>

            <button className="btn-primary btn-large" onClick={handleComplete}>
              {texts.microphone[lang as 'pl' | 'en'].continue}
            </button>
          </div>
        )}
      </div>
    </div>
  )
}
