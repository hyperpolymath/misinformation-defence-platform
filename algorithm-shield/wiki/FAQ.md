# Frequently Asked Questions (FAQ)

Quick answers to common questions about Algorithm Shield.

---

## General Questions

### What is Algorithm Shield?

Algorithm Shield is a browser extension that helps you break free from filter bubbles by making your browsing behavior unpredictable. It confuses recommendation algorithms so they can't pigeonhole you into a narrow category.

### Is this legal?

Yes. You're just using a browser extension to automate your own browsing. It's no different from using bookmarks, browser scripts, or clicking links manually.

### Will platforms ban me for using this?

No. Algorithm Shield uses normal browsing actions (clicks, scrolls, tab opens) that are indistinguishable from manual browsing. There's nothing to detect.

### Does it work like an ad blocker?

No - it's complementary. Ad blockers hide trackers. Algorithm Shield confuses recommendation algorithms. **Use both together** for maximum privacy and diversity.

---

## Privacy & Security

### Does Algorithm Shield collect my data?

**No.** Zero data collection. Everything happens locally in your browser.

- No servers
- No cloud services
- No tracking
- No analytics
- No network requests

**You can verify**: The extension works 100% offline. Disconnect from the internet and it still functions.

### Can Algorithm Shield see my browsing history?

**No.** The extension only requests these permissions:
- `storage` - Save your settings locally
- `activeTab` - See the current tab when you click the extension
- `scripting` - Inject code into supported platforms

It does NOT request access to:
- `tabs` (all tabs)
- `history`
- `cookies`
- `webNavigation`

### Is my data encrypted?

Your settings are stored in `chrome.storage.local`, which is encrypted by Chrome if you have disk encryption enabled (FileVault, BitLocker, etc.).

The extension itself doesn't send data anywhere, so there's nothing to encrypt in transit.

### Has it been audited?

- **Code audit**: You can review the source code yourself (it's open source)
- **Security audit**: Self-audited for v0.1.1 (XSS prevention, CSP, input validation)
- **Third-party audit**: Planned for v2.0 ($5-10k professional security audit)

---

## Technical Questions

### What platforms does it support?

| Platform | Status | Version |
|----------|--------|---------|
| YouTube | 🟡 In progress | v0.5 (Mar 2026) |
| X/Twitter | 🔜 Planned | v1.0 (Jun 2026) |
| Instagram | 🔜 Planned | v1.0 (Jun 2026) |
| TikTok | 🔜 Planned | v2.0 (Dec 2026) |
| Facebook | 🔜 Planned | v2.0 (Dec 2026) |

On unsupported platforms, the extension stays dormant and doesn't interfere.

### Will it slow down my browser?

**No.** The extension is very lightweight:
- **Bundle size**: <1MB total
- **WASM engine**: 180KB (extremely fast)
- **Memory**: <10MB
- **CPU**: Negligible (rule evaluation takes ~5ms)

You won't notice any performance impact.

### Does it work on mobile?

Not yet. Browser extensions aren't well-supported on mobile browsers. Planned for v3.0+ if mobile browser APIs improve.

### What is WASM?

WebAssembly (WASM) is a fast, safe binary format that runs in your browser. Algorithm Shield uses WASM for the rule engine (written in Rust) to ensure high performance and memory safety.

---

## Usage Questions

### How do I know it's working?

1. Click the Algorithm Shield icon in your toolbar
2. Click a lens (e.g., Random Walk)
3. Click "Breach Membrane"
4. You should see 3-5 new tabs open with diverse content
5. Check the browser console (F12) - you'll see "🛡️ Algorithm Shield" logs

### What's the difference between lenses and personas?

- **Lenses**: Strategies for *finding* diverse content (Opposition, Random Walk, etc.)
- **Personas**: Behavioral *profiles* that change how you appear to algorithms (Gardener, Tech Skeptic, etc.)

**Use lenses** when you want to actively explore. **Use personas** when you want the algorithm to see you differently over time.

### Can I use multiple lenses at once?

Not in v0.1.1. Only one lens can be active at a time. Lens composition is planned for v1.0.

### What does "Breach Membrane" do?

"Breach the membrane" means crossing the boundary of your filter bubble. When you click this button:
1. The active lens generates 3-5 URLs outside your bubble
2. Algorithm Shield opens these URLs in new tabs
3. The algorithm sees you engaging with diverse content
4. Your future recommendations become more diverse

### Can I customize the personas?

Custom persona creation is coming in v1.0 (June 2026). For now, you can only use the 3 built-in personas (Gardener, Tech Skeptic, Art Student).

---

## Troubleshooting

### The extension icon doesn't appear

1. Go to `chrome://extensions`
2. Verify "Algorithm Shield" is listed and enabled
3. If not listed, try loading it again (see [User Guide](User-Guide#installation))
4. If still missing, see [Troubleshooting](Troubleshooting)

### "Breach Membrane" doesn't do anything

**Possible causes**:
1. **Not on a supported platform**: Extension only works on YouTube (v0.5+). Check [supported platforms](#what-platforms-does-it-support).
2. **No lens active**: Click a lens first, then click "Breach Membrane"
3. **Content script not loaded**: Refresh the page and try again
4. **Console errors**: Press F12 and check Console tab for errors

See [Troubleshooting](Troubleshooting) for more details.

### Popup shows "Analyzing feed..."

This is normal in v0.1.1. The bubble analysis feature is coming in v0.5 (March 2026).

For now, this section is a placeholder. The extension still works - use lenses and personas as normal.

### Extension is blocked on some sites

Algorithm Shield only runs on `https://*` URLs (secure sites). It won't work on:
- `chrome://` pages (browser internals)
- `file://` pages (local files)
- `about:` pages (browser pages)

This is a Chrome security restriction.

---

## Philosophical Questions

### Isn't this manipulating the algorithm?

Yes - but you're manipulating it *on your own behalf*. The goal is to **resist profiling**, not to deceive others or game the system for personal gain.

Think of it like:
- Wearing different clothes to different events (presenting different facets of yourself)
- Reading books outside your usual interests (intellectual curiosity)
- Traveling to broaden your perspective (seeking diverse experiences)

Algorithm Shield automates this kind of intentional diversity.

### Won't this make my recommendations worse?

**It depends on what you mean by "worse"**:

- If "good recommendations" = "more of what I already like" → Yes, Algorithm Shield will "worsen" this
- If "good recommendations" = "diverse, challenging, surprising" → No, Algorithm Shield improves this

The extension is for people who value **intellectual diversity** over **personalized comfort**.

### What's wrong with personalized recommendations?

Nothing - in moderation. But when algorithms *only* show you personalized content:
- You lose exposure to opposing viewpoints
- You stop encountering new ideas
- You get stuck in an echo chamber
- Democratic discourse becomes impossible (everyone lives in a different reality)

Algorithm Shield helps you **stay diverse** while still getting some personalization.

### Is this just for political bubbles?

No! Filter bubbles affect everything:
- **Music**: Only hearing one genre
- **Food**: Only seeing recipes you've made before
- **Hobbies**: Never discovering new interests
- **News**: Only reading sources that confirm your beliefs
- **Culture**: Missing out on art, literature, films outside your usual taste

Algorithm Shield works on **any kind of filter bubble**, not just political ones.

---

## Contributing

### How can I help?

See [Contributing](Contributing) for details. Quick ideas:
- **Test** the extension and report bugs
- **Suggest** new lenses or personas
- **Add support** for new platforms (if you know web development)
- **Improve docs** (fix typos, clarify instructions)
- **Spread the word** (tell friends, write blog posts)

### I found a bug. What should I do?

1. Check [existing issues](https://github.com/hyperpolymath/algorithm-shield/issues) to see if it's already reported
2. If not, [open a new issue](https://github.com/hyperpolymath/algorithm-shield/issues/new) with:
   - What you expected to happen
   - What actually happened
   - Steps to reproduce
   - Browser version, OS, extension version
3. We'll investigate and respond

### I have a feature idea

Great! [Open a feature request](https://github.com/hyperpolymath/algorithm-shield/issues/new) and describe:
- What you want to achieve
- Why it would be useful
- Any ideas for how it could work

We review all suggestions.

---

## Next Steps

- **New user?** → [User Guide](User-Guide)
- **Want to build it?** → [Developer Guide](Developer-Guide)
- **Having issues?** → [Troubleshooting](Troubleshooting)
- **More questions?** → [GitHub Discussions](https://github.com/hyperpolymath/algorithm-shield/discussions)

---

**Last Updated**: 2026-01-24 (v0.1.1)
