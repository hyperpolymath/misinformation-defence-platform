# Algorithm Shield - Extension Testing Checklist

**Date**: 2026-01-24
**Version**: 0.1.0
**Browser**: Edge Dev (Chromium-based)

## Pre-Test Setup

**Extension Location**: `~/Documents/hyperpolymath-repos/algorithm-shield/dist/`

**Required Files** (verify all present):
- ✅ manifest.json
- ✅ background.js
- ✅ content.js
- ✅ popup.html, popup.css, popup.js
- ✅ assets/icon-16.png, icon-48.png, icon-128.png
- ✅ pkg/algorithm_shield_engine_bg.wasm (181KB)
- ✅ pkg/algorithm_shield_engine.js

## Loading the Extension

### Step 1: Open Edge Extensions Page

1. Navigate to: `edge://extensions/`
2. Enable **Developer mode** (toggle in top-right)
3. Click **"Load unpacked"**
4. Select folder: `~/Documents/hyperpolymath-repos/algorithm-shield/dist/`
5. Click **"Select Folder"**

**Expected Result**: Extension appears in list with Algorithm Shield icon

### Step 2: Verify Extension Loads

**Check for errors**:
- [ ] No red error messages in extension card
- [ ] "Errors" button does not appear (or shows 0 errors)
- [ ] Extension ID is assigned
- [ ] Version shows "0.1.0"

**If errors appear**:
1. Click "Errors" button
2. Copy error messages
3. Check browser console: `F12 → Console tab`

### Step 3: Test Service Worker (Background Script)

1. On extensions page, click **"Service worker"** link (under extension details)
2. DevTools opens for background script
3. Check Console tab for errors

**Expected Output**:
```
Algorithm Shield background service worker initialized
```

**Tests**:
- [ ] No console errors
- [ ] Service worker status: "Active"
- [ ] WASM module loads without errors

**Run in console**:
```javascript
// Test storage API
chrome.storage.local.set({test: "hello"}, () => {
  chrome.storage.local.get("test", (result) => {
    console.log("Storage test:", result.test === "hello" ? "✅ PASS" : "❌ FAIL");
  });
});

// Test WASM loading (if exposed globally)
console.log("WASM check:", typeof wasmModule !== 'undefined' ? "✅ Loaded" : "⚠️ Not loaded yet");
```

### Step 4: Test Popup

1. Click Algorithm Shield icon in toolbar (or pin it first)
2. Popup should open

**Expected Behavior**:
- [ ] Popup displays (300x400px window)
- [ ] Icon shows correctly
- [ ] HTML/CSS renders properly
- [ ] No console errors in popup DevTools

**Right-click popup → Inspect** to open DevTools for popup:

**Tests in popup console**:
```javascript
// Test chrome.storage from popup
chrome.storage.local.get(null, (data) => {
  console.log("All stored data:", data);
});

// Test chrome.tabs API
chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
  console.log("Current tab:", tabs[0].url);
});
```

### Step 5: Test Content Script Injection

1. Navigate to YouTube: https://www.youtube.com/
2. Open DevTools (F12)
3. Go to Console tab

**Expected**:
```
Algorithm Shield content script injected on youtube.com
```

**Test in console**:
```javascript
// Check if content script loaded
console.log("Content script:", typeof algorithmShieldContentScript !== 'undefined' ? "✅ Loaded" : "❌ Not loaded");
```

**Look for**:
- [ ] Content script logs appear
- [ ] No errors about missing permissions
- [ ] Script runs at document_start (before page loads)

### Step 6: Test WASM Integration

**In background worker console**:
```javascript
// Try to call WASM functions (example - adjust based on actual exports)
// Check if module loaded
console.log("WASM exports:", Object.keys(wasmExports || {}));
```

**Expected**:
- [ ] WASM file fetched successfully (check Network tab)
- [ ] Size is ~181KB (not 0 bytes)
- [ ] MIME type: application/wasm
- [ ] No CORS errors

### Step 7: Test Cross-Component Communication

**In popup console**:
```javascript
// Send message to background
chrome.runtime.sendMessage({type: "ping"}, (response) => {
  console.log("Background response:", response);
});
```

**In background console**:
```javascript
// Listen for messages
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log("Message received:", message);
  sendResponse({status: "pong"});
  return true;
});
```

**Tests**:
- [ ] Messages send successfully
- [ ] Responses received
- [ ] No permission errors

## Common Issues & Solutions

### Issue: "Service worker registration failed"
**Cause**: Syntax error in background.js or manifest
**Fix**: Check console for exact error, verify manifest.json is valid JSON

### Issue: "Failed to load extension"
**Cause**: Missing manifest_version or required fields
**Fix**: Verify manifest.json has all required fields

### Issue: WASM file not loading
**Cause**: Incorrect path or web_accessible_resources misconfigured
**Fix**: Check manifest.json web_accessible_resources matches WASM filename

### Issue: Content script not injecting
**Cause**: host_permissions or matches pattern incorrect
**Fix**: Verify manifest has "https://*/*" in host_permissions

### Issue: Icons not displaying
**Cause**: Icon files missing or wrong path
**Fix**: Check assets/icon-*.png exist in dist/

### Issue: "Cannot read property of undefined"
**Cause**: Trying to use uninitialized objects
**Fix**: Add null checks, ensure initialization order

## Test Results Log

**Date**: ___________
**Browser**: Edge Dev v___________
**Extension Version**: 0.1.0

| Test | Status | Notes |
|------|--------|-------|
| Extension loads | ⬜ | |
| No manifest errors | ⬜ | |
| Service worker active | ⬜ | |
| WASM loads (181KB) | ⬜ | |
| Popup displays | ⬜ | |
| Storage API works | ⬜ | |
| Tabs API works | ⬜ | |
| Content script injects | ⬜ | |
| Cross-component messaging | ⬜ | |
| Icons display correctly | ⬜ | |

**Overall Status**: ⬜ PASS / ⬜ FAIL

**Errors Found**:
```
(paste any error messages here)
```

**Next Steps**:
```
(what needs to be fixed based on test results)
```

## Performance Checks

### Memory Usage
1. Open: `edge://extensions/` → Click "Details" on Algorithm Shield
2. Check memory usage (should be < 50MB when idle)

### WASM Load Time
1. Check Network tab when extension loads
2. WASM should load in < 500ms

### Popup Render Time
1. Click icon, check how fast popup appears
2. Should be < 200ms

## Next Testing Phase

Once basic loading works, proceed to:
- [ ] **Functional testing**: Test lens implementations
- [ ] **Platform integration**: Test YouTube feed extraction
- [ ] **Human timing**: Test bot detection evasion
- [ ] **State persistence**: Test data survives browser restart
- [ ] **Error handling**: Test recovery from failures

---

**Testing Philosophy**: We test thoroughly because users trust us with their browsing behavior. Every permission we request, we must use correctly and safely.
