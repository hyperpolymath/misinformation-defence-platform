// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Background service worker
// Coordinates between popup, content scripts, and storage

console.log('🛡️ Algorithm Shield background worker active')

// Install handler
chrome.runtime.onInstalled.addListener((details) => {
  if (details.reason === 'install') {
    console.log('First install - initializing defaults')
    chrome.storage.local.set({
      shieldState: {
        mode: 'normal',
        activeLens: null,
        activePersona: null,
        membraneThickness: 0.5,
        isPaused: false
      }
    })
  }
})

// Message routing
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log('Background received:', message)

  if (message.type === 'GET_STATE') {
    chrome.storage.local.get('shieldState', (result) => {
      sendResponse(result.shieldState || {})
    })
    return true // Async response
  }

  if (message.type === 'LOG_ACTION') {
    // Store action in activity log
    chrome.storage.local.get('activityLog', (result) => {
      const log = result.activityLog || []
      log.push({
        ...message.action,
        timestamp: Date.now()
      })
      // Keep last 100 entries
      const trimmed = log.slice(-100)
      chrome.storage.local.set({ activityLog: trimmed })
    })
  }

  sendResponse({ status: 'ok' })
})
