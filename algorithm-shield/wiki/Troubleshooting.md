# Troubleshooting

Common issues and how to fix them.

---

## Installation Issues

### Extension doesn't appear in Chrome

**Symptoms**: After loading the extension, the icon doesn't appear in the toolbar.

**Solutions**:
1. Check that the extension is enabled:
   - Go to `chrome://extensions`
   - Find "Algorithm Shield"
   - Toggle should be **ON** (blue)

2. Pin the extension to toolbar:
   - Click the puzzle piece icon (🧩) in Chrome toolbar
   - Find "Algorithm Shield"
   - Click the pin icon

3. Verify files are intact:
   - Check that `dist/` folder contains:
     - `manifest.json`
     - `popup.html`, `popup.css`, `popup.js`
     - `content.js`, `background.js`
     - `pkg/` folder with WASM files
   - If missing, rebuild: `deno task build`

### "Manifest file is missing or unreadable" error

**Symptoms**: Chrome shows this error when loading the extension.

**Cause**: `manifest.json` is missing or corrupted.

**Solutions**:
1. Check that `dist/manifest.json` exists
2. Validate JSON syntax: `cat dist/manifest.json | jq`
3. If missing, copy from root: `cp manifest.json dist/`
4. Reload extension in Chrome

### Extension loads but popup is blank

**Symptoms**: Click the icon, popup opens but shows nothing.

**Solutions**:
1. **Check browser console**:
   - Right-click extension icon → "Inspect popup"
   - Look for JavaScript errors in Console tab
   - Common errors:
     - `popup.js not found` → Rebuild extension
     - `WASM failed to load` → Check WASM files in `dist/pkg/`
     - `Syntax error` → JavaScript file corrupted, rebuild

2. **Check file permissions**:
   - `ls -la dist/` should show readable files
   - If permission denied, run: `chmod -R 644 dist/*`

3. **Try a clean rebuild**:
   ```bash
   rm -rf dist/
   deno task build
   # Reload extension in Chrome
   ```

---

## Runtime Issues

### "Breach Membrane" button doesn't work

**Symptoms**: Click "Breach Membrane" but nothing happens.

**Diagnosis**:
1. Open YouTube (or supported platform)
2. Press F12 → Console tab
3. Click "Breach Membrane"
4. Look for error messages

**Common Causes**:

**1. Content script not loaded**
- **Error**: `Could not establish connection. Receiving end does not exist.`
- **Solution**: Refresh the page, try again
- **Prevention**: Content script should auto-load on page load

**2. Not on supported platform**
- **Error**: `Shield only works on supported platforms like YouTube and Twitter`
- **Solution**: Extension only works on YouTube (v0.5+). Go to https://youtube.com
- **Check supported platforms**: [FAQ - Supported Platforms](FAQ#what-platforms-does-it-support)

**3. No lens active**
- **Symptom**: Button clicks but nothing happens
- **Solution**: Activate a lens first (click a lens card), then click "Breach Membrane"

**4. Pop-up blocker**
- **Symptom**: No tabs open
- **Solution**: Allow pop-ups for YouTube:
  - Chrome settings → Privacy → Site settings → Pop-ups → Allow for youtube.com

### Keyboard navigation doesn't work

**Symptoms**: Can't Tab through controls, arrow keys don't navigate grids.

**Solutions**:
1. **Click inside the popup** first (to give it focus)
2. **Check accessibility module loaded**:
   - Right-click extension icon → "Inspect popup"
   - Console tab → Type: `typeof Accessibility`
   - Should show: `object`
   - If `undefined`: popup.js didn't load correctly, rebuild extension

3. **Test with simple controls**:
   - Tab key should focus "Breach Membrane" button
   - Enter/Space should activate it
   - If this works, keyboard nav is functional

### Screen reader doesn't announce changes

**Symptoms**: Using NVDA/JAWS/VoiceOver but state changes aren't announced.

**Solutions**:
1. **Verify ARIA live regions**:
   - Right-click extension icon → "Inspect popup"
   - Elements tab → Search for `aria-live`
   - Should find multiple elements
   - If missing: popup.html corrupted, rebuild

2. **Test with simple changes**:
   - Click a lens (should announce: "random-walk lens activated")
   - Click pause (should announce: "Shield paused")
   - If nothing announced: screen reader may not support extension popups

3. **Known limitation**: Some screen readers don't fully support extension popups. Try:
   - Opening the control panel (coming v0.5) instead
   - Using keyboard shortcuts (coming v0.5)

---

## Platform-Specific Issues

### YouTube: No results extracted

**Symptoms**: Console shows `🔍 Extracted 0 results from YouTube`

**Cause**: YouTube DOM changed, selectors no longer work.

**Solutions**:
1. **Check console for selector details**:
   - Look for messages like `Found 0 items with selector: ytd-rich-item-renderer`
   - This tells you which selector failed

2. **Try heuristic fallback**:
   - Heuristic extractor should activate automatically
   - Look for: `🔬 Using heuristic extraction`
   - Should find links anyway (generic approach)

3. **Report the issue**:
   - [Open a GitHub issue](https://github.com/hyperpolymath/algorithm-shield/issues/new)
   - Include:
     - YouTube URL
     - Console log (full output)
     - What you expected to see

4. **Temporary workaround**:
   - Use the extension on a different platform (when available)
   - Or manually open diverse content until selectors are fixed

### Bing/Google: Extraction works but results are wrong

**Symptoms**: Console shows results extracted, but diversity calculation is off.

**Cause**: Category classification may be inaccurate.

**Diagnosis**:
1. Check console for extracted titles:
   ```
   📋 Extracted titles: ["How to...", "Top 10...", ...]
   ```
2. Check category assignments:
   ```
   📊 Categories: {tech: 3, lifestyle: 2, education: 1}
   ```

**Solutions**:
- If categories look wrong, this is a known limitation in v0.1.1
- Category detection improves in v0.5 (machine learning-based)
- For now, lens-based diversity still works (doesn't rely on perfect categorization)

---

## Performance Issues

### Extension uses too much memory

**Symptoms**: Chrome shows high memory usage for Algorithm Shield.

**Expected**: <10MB for the extension itself.

**If higher**:
1. **Check WASM instance**:
   - WASM engine should be ~2-3MB
   - If >50MB, there may be a memory leak
   - Reload extension: `chrome://extensions` → click reload

2. **Check for tab leaks**:
   - Algorithm Shield opens tabs - make sure you close them
   - If hundreds of tabs open: close old tabs, restart browser

3. **Disable if not using**:
   - Click "Pause Shield" when not actively breaching
   - Or disable extension: `chrome://extensions` → toggle OFF

### Browser feels slow after installing

**Symptoms**: Chrome is sluggish, pages load slowly.

**Unlikely to be Algorithm Shield** (extension is very lightweight), but to verify:

1. **Disable Algorithm Shield**:
   - `chrome://extensions` → toggle OFF
   - Restart Chrome
   - Is it still slow? Then not Algorithm Shield.

2. **Check other extensions**:
   - Disable all other extensions
   - Re-enable one at a time to find the culprit

3. **Check content script**:
   - Algorithm Shield's content script runs on `https://*`
   - If a specific site is slow:
     - Press F12 → Performance tab
     - Record profile
     - Look for "algorithm-shield" in flamegraph
     - If significant time: report issue

---

## Build Issues (Developers)

### ReScript compilation fails

**Error**: `rescript: command not found` or compilation errors.

**Solutions**:
1. **Install ReScript**:
   ```bash
   npm install -g rescript
   ```

2. **Check version**:
   ```bash
   rescript -v
   # Should be 11.0+
   ```

3. **Clear build cache**:
   ```bash
   npx rescript clean
   npx rescript build
   ```

4. **Check for syntax errors**:
   - ReScript is strict - check error messages carefully
   - Common issues:
     - Missing type annotations
     - Incorrect module paths
     - Syntax errors (missing `;`, `)`, etc.)

### WASM build fails

**Error**: `wasm-pack: command not found` or Rust compilation errors.

**Solutions**:
1. **Install wasm-pack**:
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

2. **Add WASM target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Check Rust version**:
   ```bash
   rustc --version
   # Should be 1.70+
   ```

4. **Build with verbose output**:
   ```bash
   cd src/rust
   wasm-pack build --target web --out-dir ../../dist/pkg -- --verbose
   ```

5. **Common errors**:
   - `linking with 'rust-lld' failed` → Update Rust: `rustup update`
   - `crate 'wasm_bindgen' not found` → Run `cargo update`

### Deno build task fails

**Error**: `deno task build` fails with errors.

**Solutions**:
1. **Check Deno version**:
   ```bash
   deno --version
   # Should be 1.40+
   ```

2. **Update Deno**:
   ```bash
   deno upgrade
   ```

3. **Check deno.json**:
   - Verify `deno.json` exists in project root
   - Check task definitions are valid

4. **Run build steps manually**:
   - See [Developer Guide - Manual Build](Developer-Guide#build-steps)

---

## Still Having Issues?

### Before Asking for Help

1. **Search existing issues**: https://github.com/hyperpolymath/algorithm-shield/issues
2. **Check FAQ**: [FAQ](FAQ)
3. **Try a clean rebuild**:
   ```bash
   git pull origin main
   rm -rf dist/ node_modules/
   deno task build
   ```
4. **Test in a clean Chrome profile**:
   - Create new Chrome profile
   - Load extension
   - See if issue persists

### Reporting a Bug

[Open a GitHub issue](https://github.com/hyperpolymath/algorithm-shield/issues/new) with:

**Required Information**:
- **Extension version**: (check `manifest.json` → `version`)
- **Browser**: Chrome/Edge/Brave + version number
- **Operating System**: macOS/Windows/Linux + version
- **Platform**: YouTube/Bing/Google/etc.
- **Steps to reproduce**:
  1. Step one
  2. Step two
  3. What happened
- **Expected behavior**: What should have happened
- **Console logs**: Copy/paste from browser console (F12)
- **Screenshots**: If relevant

**Optional but Helpful**:
- Error messages (full text)
- When did it start happening?
- Does it happen in incognito mode?
- Does it happen with all other extensions disabled?

---

## Emergency Fixes

### Extension is completely broken

```bash
# Nuclear option: full reset
cd algorithm-shield
git fetch origin
git reset --hard origin/main
rm -rf dist/ node_modules/ src/rust/target/
deno task build
# Reload in Chrome
```

### Can't uninstall extension

1. Close Chrome completely
2. Delete extension data manually:
   - macOS: `~/Library/Application Support/Google/Chrome/Default/Extensions/`
   - Windows: `%LOCALAPPDATA%\Google\Chrome\User Data\Default\Extensions\`
   - Linux: `~/.config/google-chrome/Default/Extensions/`
3. Restart Chrome

---

## Resources

- **[User Guide](User-Guide)** - Installation and usage
- **[Developer Guide](Developer-Guide)** - Build from source
- **[FAQ](FAQ)** - Common questions
- **[GitHub Issues](https://github.com/hyperpolymath/algorithm-shield/issues)** - Report bugs

---

**Last Updated**: 2026-01-24 (v0.1.1)
