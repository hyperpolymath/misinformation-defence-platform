# Developer Guide

**Learn how to build, develop, and contribute to Algorithm Shield.**

---

## 🎯 What You'll Learn

- [Quick Start](#quick-start)
- [Development Environment](#development-environment)
- [Building from Source](#building-from-source)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Contributing](#contributing)

---

## Quick Start

**Prerequisites**:
- Git
- Deno (1.40+) OR Node.js (16+) for ReScript compiler
- Rust + wasm-pack
- Chrome/Chromium for testing

**One-line setup**:
```bash
git clone https://github.com/hyperpolymath/algorithm-shield.git
cd algorithm-shield
deno task build  # OR: npm install && npx rescript build
```

**Load extension**:
1. Open `chrome://extensions`
2. Enable "Developer mode"
3. Click "Load unpacked" → select `dist/` folder

---

## Development Environment

### Required Tools

| Tool | Version | Purpose |
|------|---------|---------|
| **Deno** | 1.40+ | Build tooling (preferred) |
| **ReScript** | Latest | UI compilation |
| **Rust** | 1.70+ | WASM rule engine |
| **wasm-pack** | 0.12+ | Rust → WASM compilation |
| **Git** | 2.0+ | Version control |

### Installation

**macOS/Linux**:
```bash
# Install Deno
curl -fsSL https://deno.land/install.sh | sh

# Install ReScript (via npm)
npm install -g rescript

# Install Rust + wasm-pack
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Add WASM target
rustup target add wasm32-unknown-unknown
```

**Windows**:
```powershell
# Install Deno
irm https://deno.land/install.ps1 | iex

# Install ReScript
npm install -g rescript

# Install Rust from https://rustup.rs
# Install wasm-pack from https://rustwasm.github.io/wasm-pack/installer
```

---

## Building from Source

### Clone Repository

```bash
git clone https://github.com/hyperpolymath/algorithm-shield.git
cd algorithm-shield
```

### Build Steps

**Option 1: Full Build (Deno)**
```bash
deno task build
```

**Option 2: Manual Build**
```bash
# 1. Install dependencies
npm install

# 2. Compile ReScript
npx rescript build

# 3. Build Rust → WASM
cd src/rust
wasm-pack build --target web --out-dir ../../dist/pkg
cd ../..

# 4. Copy files to dist/
cp src/ui/*.html dist/
cp src/ui/*.css dist/
cp src/ui/*.js dist/
cp manifest.json dist/
```

### Build Output

```
dist/
├── manifest.json          # Extension manifest
├── popup.html            # Popup UI
├── popup.css             # Styles (WCAG AAA compliant)
├── popup.js              # UI logic (with security/accessibility modules)
├── content.js            # Content script
├── background.js         # Service worker
├── metrics-calculator.js # Bubble metrics
├── heuristic-extractor.js # DOM extraction fallback
├── wasm-loader.js        # WASM integration
├── pkg/                  # WASM files
│   ├── algorithm_shield_engine_bg.wasm  # (180KB)
│   └── algorithm_shield_engine.js       # WASM bindings
└── assets/               # Icons
    ├── icon-16.png
    ├── icon-48.png
    └── icon-128.png
```

---

## Project Structure

```
algorithm-shield/
├── src/
│   ├── rescript/          # ReScript UI code
│   │   ├── actuator/      # Action execution (tabs, clicks)
│   │   ├── bindings/      # Chrome API bindings
│   │   ├── lens/          # Lens implementations
│   │   ├── membrane/      # Core membrane logic
│   │   ├── observer/      # Feed observation
│   │   └── persona/       # Persona behaviors
│   ├── rust/              # Rust rule engine
│   │   └── src/
│   │       ├── lib.rs     # WASM exports
│   │       └── minikaren/ # Rule evaluation
│   ├── platforms/         # Platform-specific adapters
│   │   ├── youtube/       # YouTube DOM extraction
│   │   ├── twitter/       # X/Twitter (coming v1.0)
│   │   └── instagram/     # Instagram (coming v1.0)
│   ├── ui/                # UI assets
│   ├── content.js         # Content script (injected)
│   ├── background.js      # Service worker
│   └── heuristic-extractor.js  # Generic DOM extraction
├── dist/                  # Build output (load this in Chrome)
├── docs/                  # Documentation
│   ├── ROADMAP.adoc       # Version roadmap
│   ├── SEAM-ANALYSIS.adoc # Technical seam analysis
│   ├── SECURITY-ACCESSIBILITY-CHECKLIST.adoc
│   └── CRYPTOGRAPHIC-SUITE.adoc  # Security spec
├── wiki/                  # GitHub wiki pages
├── STATE.scm              # Current project state
├── META.scm               # Architectural decisions (ADRs)
├── ECOSYSTEM.scm          # Position in hyperpolymath ecosystem
├── ARCHITECTURE.md        # Architecture overview
├── README.adoc            # Project README
├── CONTRIBUTING.adoc      # Contribution guidelines
└── manifest.json          # Extension manifest (copied to dist/)
```

---

## Development Workflow

### Typical Development Cycle

1. **Make changes** to source files (`.res`, `.rs`, `.js`)
2. **Rebuild**:
   ```bash
   # ReScript only (fast)
   npx rescript build

   # Or full rebuild (includes WASM)
   deno task build
   ```
3. **Reload extension** in Chrome:
   - Go to `chrome://extensions`
   - Click reload button on Algorithm Shield card
4. **Test changes** on supported platform (YouTube)
5. **Check console** for errors (F12 → Console)

### Hot Reload

**For UI changes** (HTML/CSS/JS):
- Edit files in `src/ui/` or `dist/` directly
- Reload extension in Chrome
- Changes appear immediately

**For ReScript changes**:
```bash
# Watch mode (auto-recompiles on save)
npx rescript build -w
```

**For Rust/WASM changes**:
```bash
# Must rebuild WASM manually
cd src/rust
wasm-pack build --target web --out-dir ../../dist/pkg
```

---

## Testing

### Manual Testing

**Load Extension**:
1. Build: `deno task build`
2. Load `dist/` folder in Chrome
3. Open extension popup
4. Verify UI loads without errors

**Test on YouTube**:
1. Go to https://youtube.com
2. Open extension popup
3. Click "Breach Membrane"
4. Check console for extraction logs
5. Verify tabs open with diverse content

### Automated Testing (Coming v0.5)

```bash
# Run unit tests (not yet implemented)
deno task test

# Run integration tests (not yet implemented)
deno task test:integration
```

### Debugging

**Enable verbose logging**:
```javascript
// In dist/popup.js or dist/content.js
console.log('🔍 Debug:', variableName)
```

**Check extension logs**:
1. Open `chrome://extensions`
2. Click "Errors" button on Algorithm Shield
3. Or: Right-click extension icon → Inspect popup → Console

**Check content script logs**:
1. Open YouTube (or supported platform)
2. Press F12 → Console tab
3. Look for "🛡️ Algorithm Shield" messages

---

## Contributing

### Before You Start

1. **Read** [CONTRIBUTING.adoc](https://github.com/hyperpolymath/algorithm-shield/blob/main/CONTRIBUTING.adoc)
2. **Check** [open issues](https://github.com/hyperpolymath/algorithm-shield/issues)
3. **Join** discussions on GitHub

### Contribution Workflow

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feature/your-feature`
3. **Make changes** and commit:
   ```bash
   git add .
   git commit -m "feat: Add your feature

   Detailed description of what changed and why.

   Co-Authored-By: Your Name <your@email.com>"
   ```
4. **Push** to your fork: `git push origin feature/your-feature`
5. **Open a Pull Request** on GitHub

### Commit Message Format

```
type: Short description (50 chars max)

Longer explanation of what changed and why.
Include context, motivation, and any breaking changes.

Co-Authored-By: Your Name <your@email.com>
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### Code Style

**ReScript**:
- Use descriptive variable names
- 2-space indentation
- Type annotations for complex functions
- Format with: `npx rescript format`

**Rust**:
- Follow Rust conventions
- Use `rustfmt`: `cargo fmt`
- Use `clippy`: `cargo clippy`

**JavaScript**:
- 2-space indentation
- Semicolons required
- JSDoc comments for public functions

**Security**:
- All inputs MUST be validated
- Use `Security.sanitizeText()` for user-facing text
- Use `Security.sanitizeHTML()` for dynamic HTML
- Never use `eval()` or `innerHTML` with untrusted data

**Accessibility**:
- All interactive elements need `aria-label`
- Maintain AAA contrast (7:1 minimum)
- Test with keyboard-only navigation
- Test with screen reader (NVDA/JAWS/VoiceOver)

---

## Architecture Overview

### Technology Stack

- **UI**: ReScript (type-safe, functional)
- **Rule Engine**: Rust/WASM (performance, safety)
- **Security**: Input validation, XSS prevention, strict CSP
- **Accessibility**: WCAG 2.3 AAA (18/18 criteria met)
- **Build**: Deno (no Node/npm per RSR policy)

### Key Architectural Decisions

See [META.scm](https://github.com/hyperpolymath/algorithm-shield/blob/main/META.scm) for full ADR list.

**ADR-001**: ReScript for UI (type safety, RSR compliance)
**ADR-002**: Rust/WASM for rule engine (performance)
**ADR-003**: Manifest v3 (Chrome requirement)
**ADR-004**: Deno for builds (RSR compliance)
**ADR-007**: Hybrid Ephapax/Rust (v2.0+, profile-guided)

### Critical Seams

See [docs/SEAM-ANALYSIS.adoc](https://github.com/hyperpolymath/algorithm-shield/blob/main/docs/SEAM-ANALYSIS.adoc) for detailed analysis.

| Seam | Risk | Status |
|------|------|--------|
| ReScript ↔ WASM | HIGH | ✅ CLOSED |
| Popup ↔ State | MEDIUM | ✅ SEALED |
| Content ↔ DOM | CRITICAL | 🟡 In Progress |
| Actuator ↔ Detection | CRITICAL | 🟡 In Progress |

---

## Advanced Topics

### Adding a New Platform

1. Create adapter in `src/platforms/newplatform/`
2. Implement `PlatformAdapter.res` interface
3. Add platform detection in `src/content.js`
4. Test DOM extraction thoroughly
5. Document in README

Example structure:
```
src/platforms/newplatform/
├── NewPlatformAdapter.res      # Main adapter
├── NewPlatformDOMBindings.res  # DOM queries
└── NewPlatformSignals.res      # Signal extraction
```

### Adding a New Lens

1. Add lens definition in `src/rescript/lens/Lens.res`
2. Implement URL generation logic
3. Update popup UI (`dist/popup.html`)
4. Test with breach action
5. Document in User Guide

### Adding Security Features

1. Add validation function to `Security` module (`dist/popup.js`)
2. Use validation before any storage/message operations
3. Add tests for edge cases
4. Document in `docs/SECURITY-ACCESSIBILITY-CHECKLIST.adoc`

---

## Useful Commands

```bash
# Development
npx rescript build -w          # Watch ReScript files
deno task build               # Full rebuild
deno task clean               # Clean build artifacts

# Rust
cd src/rust
cargo build --release         # Build Rust (native)
wasm-pack build              # Build WASM
cargo test                   # Run tests
cargo fmt                    # Format code
cargo clippy                 # Lint code

# Git
git status                   # Check changes
git add -A                   # Stage all
git commit -m "msg"          # Commit
git push                     # Push to remote
```

---

## Resources

- **[Architecture](Architecture)** - System design overview
- **[API Reference](API-Reference)** - Code documentation
- **[Roadmap](https://github.com/hyperpolymath/algorithm-shield/blob/main/docs/ROADMAP.adoc)** - Version evolution
- **[State](https://github.com/hyperpolymath/algorithm-shield/blob/main/STATE.scm)** - Current project status

---

## Getting Help

- **Questions**: [GitHub Discussions](https://github.com/hyperpolymath/algorithm-shield/discussions)
- **Bugs**: [GitHub Issues](https://github.com/hyperpolymath/algorithm-shield/issues)
- **Security**: See [SECURITY.md](https://github.com/hyperpolymath/algorithm-shield/blob/main/SECURITY.md)

---

**Last Updated**: 2026-01-24 (v0.1.1)
