// Heuristic Content Extractor
// ===========================
//
// PURPOSE:
//   Extract search results, products, or feed items from ANY website
//   WITHOUT knowing the specific HTML structure or class names.
//
// HOW IT WORKS:
//   1. Find all links on the page that look like titles (substantial text)
//   2. Group them by parent structure (repeated patterns)
//   3. Filter out navigation/UI links
//   4. Return the most likely result set
//
// WHY THIS IS BETTER THAN SITE-SPECIFIC SELECTORS:
//   - Works on sites we've never seen before
//   - Resilient to HTML changes
//   - No maintenance required per-site
//   - Handles long tail of shopping/search sites
//
// TRANSPARENCY:
//   - Logs what patterns it found
//   - Shows confidence score for results
//   - You can see what it extracted

/**
 * Extract content using heuristic pattern matching
 *
 * @returns {Array} Array of {title, description, source, confidence}
 */
function extractByHeuristics() {
  console.log('🔬 Using heuristic extraction (no site-specific knowledge)...');

  // Step 1: Find all links that might be titles
  const allLinks = Array.from(document.querySelectorAll('a[href]'));
  console.log(`📊 Found ${allLinks.length} total links on page`);

  // Step 2: Filter to links that look like content (not navigation)
  const contentLinks = allLinks.filter(link => {
    const text = link.textContent?.trim() || '';
    const href = link.href;

    // Skip if no text
    if (text.length < 10) return false;

    // Skip navigation links (too short or common nav patterns)
    const navPatterns = /^(home|about|contact|login|sign in|cart|account|help|menu|more|next|previous)$/i;
    if (navPatterns.test(text)) return false;

    // Skip if link goes to different domain (usually ads/external)
    try {
      const linkDomain = new URL(href).hostname;
      const currentDomain = window.location.hostname;
      // Allow subdomains (e.g., www.google.com and google.com)
      const baseDomain = domain => domain.split('.').slice(-2).join('.');
      if (baseDomain(linkDomain) !== baseDomain(currentDomain)) {
        return false;
      }
    } catch (e) {
      return false; // Invalid URL
    }

    // Skip if hidden (display: none, visibility: hidden)
    const style = window.getComputedStyle(link);
    if (style.display === 'none' || style.visibility === 'hidden') return false;

    // This looks like a content link!
    return true;
  });

  console.log(`📋 Filtered to ${contentLinks.length} content links`);

  if (contentLinks.length === 0) {
    console.warn('❌ No content links found');
    return [];
  }

  // Step 3: Group links by parent structure
  // Links that are siblings or have similar parent structure are likely results
  const groups = groupByParentStructure(contentLinks);
  console.log(`📦 Found ${groups.length} groups of similar links`);

  // Step 4: Find the largest group (most likely the main results)
  const largestGroup = groups.reduce((max, group) =>
    group.length > max.length ? group : max, groups[0]
  );

  console.log(`✅ Selected largest group: ${largestGroup.length} items`);

  // Step 5: Extract titles and descriptions
  const results = largestGroup.map(link => {
    const title = link.textContent?.trim() || '';

    // Try to find a description nearby (sibling or parent's text)
    let description = '';
    const parent = link.closest('div, article, li, section');
    if (parent) {
      // Get text from parent but exclude the title
      const parentText = parent.textContent?.trim() || '';
      description = parentText.replace(title, '').trim().slice(0, 200);
    }

    return {
      title,
      description,
      url: link.href,
      source: 'heuristic',
      confidence: calculateConfidence(link, largestGroup.length)
    };
  });

  console.log(`🎯 Extracted ${results.length} items with heuristic extractor`);
  return results;
}

/**
 * Group links by their parent structure
 * Links with similar parent tags/hierarchy are likely part of the same result set
 */
function groupByParentStructure(links) {
  const groups = {};

  for (const link of links) {
    // Get the parent signature (tag names up the tree)
    const signature = getParentSignature(link, 3); // Look 3 levels up

    if (!groups[signature]) {
      groups[signature] = [];
    }
    groups[signature].push(link);
  }

  // Return groups as array, sorted by size
  return Object.values(groups)
    .filter(group => group.length >= 3) // Must have at least 3 items to be a "group"
    .sort((a, b) => b.length - a.length);
}

/**
 * Get a signature of parent elements
 * Example: "div > article > div > a" becomes "div.article.div"
 */
function getParentSignature(element, depth) {
  const tags = [];
  let current = element;

  for (let i = 0; i < depth && current.parentElement; i++) {
    current = current.parentElement;
    tags.push(current.tagName.toLowerCase());
  }

  return tags.join('.');
}

/**
 * Calculate confidence score for an extracted item
 * Higher confidence = more likely to be a real result
 */
function calculateConfidence(link, groupSize) {
  let confidence = 0.5; // Base confidence

  // Larger groups = higher confidence
  if (groupSize > 10) confidence += 0.2;
  else if (groupSize > 5) confidence += 0.1;

  // Longer title text = higher confidence (more likely to be content)
  const textLength = link.textContent?.trim().length || 0;
  if (textLength > 50) confidence += 0.2;
  else if (textLength > 30) confidence += 0.1;

  // Has heading tag (h1-h6) = higher confidence
  const hasHeading = link.querySelector('h1, h2, h3, h4, h5, h6') ||
                      link.closest('h1, h2, h3, h4, h5, h6');
  if (hasHeading) confidence += 0.1;

  return Math.min(confidence, 1.0); // Cap at 1.0
}

// Export for use in content.js
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { extractByHeuristics };
}
