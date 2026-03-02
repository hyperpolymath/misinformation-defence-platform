# User Guide

**Learn how to install and use Algorithm Shield to break free from filter bubbles.**

---

## 🎯 What You'll Learn

- [Installation](#installation)
- [Basic Usage](#basic-usage)
- [Understanding the Interface](#understanding-the-interface)
- [Using Lenses](#using-lenses)
- [Using Personas](#using-personas)
- [Advanced Features](#advanced-features)

---

## Installation

### Chrome / Edge / Brave

**Option 1: Chrome Web Store** (Coming soon - v1.0)

_Extension not yet published. Use Option 2 for now._

**Option 2: Load Unpacked Extension**

1. Download the latest release from [GitHub Releases](https://github.com/hyperpolymath/algorithm-shield/releases)
2. Extract the ZIP file
3. Open Chrome and go to `chrome://extensions`
4. Enable **Developer mode** (toggle in top-right)
5. Click **Load unpacked**
6. Select the `dist/` folder from the extracted files
7. The Algorithm Shield icon should appear in your toolbar

**Option 3: Build from Source**

See the [Developer Guide](Developer-Guide) for instructions.

### Firefox

_Firefox support planned for v1.0 (June 2026)_

---

## Basic Usage

### Opening the Extension

Click the **🛡️ Algorithm Shield** icon in your browser toolbar to open the popup.

### The Main View

The popup shows:
- **Membrane Thickness** - How aggressively the shield crosses filter bubbles (0.0 - 1.0)
- **Bubble Analysis** - Information about your current filter bubble
- **Quick Actions** - Breach membrane, pause shield
- **Lenses** - Different strategies for discovering diverse content
- **Personas** - Behavioral profiles to confuse profiling algorithms

---

## Understanding the Interface

### Membrane Thickness Bar

The blue bar shows how "thick" your membrane is:
- **Low (0.3-0.4)**: Gentle - occasional diverse content
- **Medium (0.5-0.6)**: Balanced - regular bubble crossing
- **High (0.7-0.8)**: Aggressive - frequent diverse content

**Tip**: Start with medium (0.5) and adjust based on preference.

### Bubble Analysis (Coming in v0.5)

This section will show:
- What categories dominate your feed
- How diverse your current content is
- Suggestions for crossing the membrane

_Currently shows: "Analyzing feed..."_

---

## Using Lenses

Lenses are different strategies for finding content outside your bubble.

### Available Lenses

**⚔️ Opposition**
- Finds underrepresented viewpoints
- Shows you opposing perspectives on topics you engage with
- Example: If you watch liberal political content, shows conservative viewpoints

**🎲 Random Walk**
- Completely random diverse topics
- Example: If you're into tech, might show origami, mycology, or Esperanto
- Best for: Breaking out of deep bubbles

**⏳ Time-Shift**
- Explores timeless classic content
- Example: Victorian literature, 1960s music, ancient philosophy
- Best for: Escaping the current moment

**✨ Serendipity**
- Pure randomness - utterly unexpected
- Example: Anything from competitive dog grooming to underwater basket weaving
- Best for: Maximum surprise

### How to Use a Lens

1. Click on a lens card (e.g., **Random Walk**)
2. The lens activates (card turns blue)
3. Membrane thickness increases to 0.7
4. Click **⚡ Breach Membrane** to open diverse content
5. Algorithm Shield opens 3-5 tabs with diverse content
6. Click the lens again to deactivate

**Keyboard Shortcut**: Use arrow keys (←→↑↓) to navigate lenses

---

## Using Personas

Personas are behavioral profiles that change how you appear to algorithms.

### Available Personas

**🌱 Gardener**
- Interested in: Plants, sustainability, cooking, slow living
- Best for: Escaping tech/politics bubbles

**🔒 Tech Skeptic**
- Interested in: Privacy, digital minimalism, offline activities
- Best for: Balancing tech-heavy feeds

**🎨 Art Student**
- Interested in: Visual arts, culture, creativity, museums
- Best for: Adding cultural depth to feeds

### How to Use a Persona

1. Click on a persona card (e.g., **Gardener**)
2. The persona activates (card turns blue)
3. Your behavior patterns change to match the persona
4. Algorithm Shield automatically engages with persona-relevant content
5. Click the persona again to return to normal mode

**What Happens**:
- Extension occasionally clicks content related to the persona
- Tabs may open in the background (you can close them)
- The algorithm starts seeing you as a different "type" of person

**Privacy**: All persona actions happen locally - no data leaves your device

---

## Advanced Features

### Pause Shield

Click **⏸️ Pause Shield** to temporarily disable all filter bubble protection.

**When to use**:
- You want normal recommendations for a while
- Troubleshooting website issues
- Comparing feed with/without shield

Click **▶️ Resume Shield** to re-enable.

### Opening the Control Panel

Click **Open Control Panel** at the bottom of the popup for:
- Detailed activity log (coming in v0.5)
- Bubble map visualization (coming in v0.5)
- Custom persona creation (coming in v1.0)
- Rule customization (coming in v1.0)

---

## Tips & Best Practices

### Getting Started

1. **Start simple**: Activate one lens (try Random Walk)
2. **Breach once**: Click "Breach Membrane" and see what happens
3. **Explore**: Check out the tabs that opened
4. **Adjust**: If too random, try a different lens

### For Best Results

- **Use personas sparingly**: Switch every few days, not every hour
- **Don't over-breach**: 1-2 breaches per day is enough
- **Combine lenses**: Try Opposition lens on YouTube, Random Walk on Twitter
- **Be patient**: It takes a few days for the algorithm to notice changes

### Privacy Notes

- **All local**: Nothing leaves your browser
- **No tracking**: We don't see what you click
- **No servers**: Extension works 100% offline
- **Open source**: You can audit the code

---

## Supported Platforms

| Platform | Status | Available In |
|----------|--------|--------------|
| YouTube | 🟡 In progress | v0.5 (Mar 2026) |
| X (Twitter) | 🔜 Planned | v1.0 (Jun 2026) |
| Instagram | 🔜 Planned | v1.0 (Jun 2026) |
| TikTok | 🔜 Planned | v2.0 (Dec 2026) |
| Facebook | 🔜 Planned | v2.0 (Dec 2026) |

**Note**: Extension only works on supported platforms. On other sites, it stays dormant.

---

## Accessibility Features

Algorithm Shield is designed for **universal access**:

- **Keyboard-only navigation**: Tab through controls, use arrow keys in grids
- **Screen reader support**: Full ARIA labels and announcements
- **High contrast mode**: Adapts to your OS preferences
- **Reduced motion**: Disables animations if you have vestibular sensitivity
- **Context help**: Every control has a tooltip (hover or focus to see)
- **200% zoom**: Works perfectly at high zoom levels

**Certified WCAG 2.3 AAA compliant** (18/18 criteria met)

---

## Need Help?

- **Common issues**: See [Troubleshooting](Troubleshooting)
- **Questions**: Check the [FAQ](FAQ)
- **Bug reports**: [GitHub Issues](https://github.com/hyperpolymath/algorithm-shield/issues)
- **Feature requests**: [GitHub Issues](https://github.com/hyperpolymath/algorithm-shield/issues)

---

## What's Next?

Ready to learn more about how Algorithm Shield works under the hood?

→ [Architecture](Architecture) - Technical overview
→ [Developer Guide](Developer-Guide) - Build from source

---

**Last Updated**: 2026-01-24 (v0.1.1)
