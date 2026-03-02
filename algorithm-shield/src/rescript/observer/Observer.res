// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Observer module: Watches and classifies content in feeds

module ContentSignal = {
  type category =
    | Tech
    | Politics
    | Art
    | Science
    | Entertainment
    | News
    | Social
    | Commerce
    | Education
    | Health
    | Unknown

  type t = {
    url: string,
    text: option<string>,
    categories: array<category>,
    timestamp: float,
    platformSource: string,
    confidence: float,
  }
}

module FeedState = {
  type cluster = {
    category: ContentSignal.category,
    strength: float, // 0.0 - 1.0
  }

  type t = {
    dominantClusters: array<cluster>,
    diversity: float, // 0.0 (homogeneous) - 1.0 (diverse)
    recentSignals: array<ContentSignal.t>,
    lastUpdated: float,
  }

  let empty = {
    dominantClusters: [],
    diversity: 0.0,
    recentSignals: [],
    lastUpdated: 0.0,
  }
}

// Platform detection
let detectPlatform = (url: string): option<string> => {
  if url->String.includes("youtube.com") {
    Some("youtube")
  } else if url->String.includes("twitter.com") || url->String.includes("x.com") {
    Some("twitter")
  } else if url->String.includes("instagram.com") {
    Some("instagram")
  } else if url->String.includes("tiktok.com") {
    Some("tiktok")
  } else {
    None
  }
}

// Extract content signals from DOM
// Platform-specific implementations will be in src/platforms/
let extractSignals = (_platform: string, _document: 'a): array<ContentSignal.t> => {
  // Placeholder - actual implementation delegated to platform modules
  []
}

// Analyze feed diversity
let analyzeDiversity = (signals: array<ContentSignal.t>): float => {
  if Array.length(signals) == 0 {
    0.0
  } else {
    // Count unique categories
    let uniqueCategories = signals
      ->Array.map(s => s.categories)
      ->Array.flat
      ->Set.fromArray
      ->Set.size
      ->Int.toFloat

    // Normalize to 0.0 - 1.0
    let maxCategories = 11.0 // Number of category variants
    uniqueCategories /. maxCategories
  }
}

// Identify dominant clusters
let identifyClusters = (signals: array<ContentSignal.t>): array<FeedState.cluster> => {
  // Count category occurrences
  let categoryMap = Map.make()

  signals->Array.forEach(signal => {
    signal.categories->Array.forEach(cat => {
      let catStr = switch cat {
      | Tech => "tech"
      | Politics => "politics"
      | Art => "art"
      | Science => "science"
      | Entertainment => "entertainment"
      | News => "news"
      | Social => "social"
      | Commerce => "commerce"
      | Education => "education"
      | Health => "health"
      | Unknown => "unknown"
      }

      let current = categoryMap->Map.get(catStr)->Option.getOr(0)
      categoryMap->Map.set(catStr, current + 1)
    })
  })

  // Convert to clusters with strength
  let _total = Array.length(signals)->Int.toFloat
  []  // Simplified for now
}

// Update feed state with new signals
let updateFeedState = (
  state: FeedState.t,
  newSignals: array<ContentSignal.t>
): FeedState.t => {
  let allSignals = Array.concat(state.recentSignals, newSignals)
  let totalLen = Array.length(allSignals)
  let startIdx = if totalLen > 50 { totalLen - 50 } else { 0 }
  let recentWindow = allSignals->Array.slice(~start=startIdx, ~end=totalLen) // Keep last 50

  {
    dominantClusters: identifyClusters(recentWindow),
    diversity: analyzeDiversity(recentWindow),
    recentSignals: recentWindow,
    lastUpdated: Date.now(),
  }
}
