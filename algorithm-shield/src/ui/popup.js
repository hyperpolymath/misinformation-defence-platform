// Popup UI controller (minimal JS glue - most logic in ReScript)

// This will interface with the compiled ReScript code
// For now, basic DOM manipulation until ReScript bindings are ready

let state = {
  mode: 'normal',
  activeLens: null,
  activePersona: null,
  membraneThickness: 0.5,
  isPaused: false
}

function updateUI() {
  // Update mode indicator
  const modeEl = document.getElementById('mode-indicator')
  if (modeEl) {
    modeEl.textContent = state.activePersona
      ? `Persona: ${state.activePersona}`
      : state.mode.charAt(0).toUpperCase() + state.mode.slice(1)
  }

  // Update thickness bar
  const thicknessFill = document.getElementById('thickness-fill')
  const thicknessValue = document.getElementById('thickness-value')
  if (thicknessFill && thicknessValue) {
    thicknessFill.style.width = `${state.membraneThickness * 100}%`
    thicknessValue.textContent = state.membraneThickness.toFixed(2)
  }

  // Update lens active states
  document.querySelectorAll('.lens-card').forEach(card => {
    const lens = card.dataset.lens
    if (lens === state.activeLens) {
      card.classList.add('active')
    } else {
      card.classList.remove('active')
    }
  })

  // Update persona active states
  document.querySelectorAll('.persona-card').forEach(card => {
    const persona = card.dataset.persona
    if (persona === state.activePersona) {
      card.classList.add('active')
    } else {
      card.classList.remove('active')
    }
  })

  // Sync to storage
  chrome.storage.local.set({ shieldState: state })
}

// Event handlers
document.addEventListener('DOMContentLoaded', () => {
  // Load saved state
  chrome.storage.local.get('shieldState', (result) => {
    if (result.shieldState) {
      state = { ...state, ...result.shieldState }
      updateUI()
    }
  })

  // Breach button
  const breachBtn = document.getElementById('breach-btn')
  breachBtn?.addEventListener('click', () => {
    // Send message to content script to trigger breach
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
      chrome.tabs.sendMessage(tabs[0].id, {
        type: 'TRIGGER_BREACH'
      })
    })
  })

  // Pause button
  const pauseBtn = document.getElementById('pause-btn')
  pauseBtn?.addEventListener('click', () => {
    state.isPaused = !state.isPaused
    pauseBtn.textContent = state.isPaused ? '▶️ Resume Shield' : '⏸️ Pause Shield'
    updateUI()
  })

  // Lens cards
  document.querySelectorAll('.lens-card').forEach(card => {
    card.addEventListener('click', () => {
      const lens = card.dataset.lens
      if (state.activeLens === lens) {
        state.activeLens = null
        state.membraneThickness = 0.5
      } else {
        state.activeLens = lens
        state.membraneThickness = 0.7
      }
      updateUI()

      // Notify content script
      chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
        chrome.tabs.sendMessage(tabs[0].id, {
          type: 'ACTIVATE_LENS',
          lens: state.activeLens
        })
      })
    })
  })

  // Persona cards
  document.querySelectorAll('.persona-card').forEach(card => {
    card.addEventListener('click', () => {
      const persona = card.dataset.persona
      if (state.activePersona === persona) {
        state.activePersona = null
        state.mode = 'normal'
        state.membraneThickness = 0.5
      } else {
        state.activePersona = persona
        state.mode = 'persona'
        state.membraneThickness = 0.6
      }
      updateUI()

      // Notify content script
      chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
        chrome.tabs.sendMessage(tabs[0].id, {
          type: 'ACTIVATE_PERSONA',
          persona: state.activePersona
        })
      })
    })
  })
})
