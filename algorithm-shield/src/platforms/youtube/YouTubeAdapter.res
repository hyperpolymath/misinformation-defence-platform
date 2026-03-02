// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield

// YouTube platform adapter - DOM signal extraction

// Import DOM bindings
module Dom = YouTubeDOMBindings

// YouTube-specific DOM selectors (as of 2026-01)
module Selectors = {
  // Main feed container
  let feedContainer = "#contents.ytd-rich-grid-renderer"

  // Individual video items
  let videoItem = "ytd-rich-item-renderer"

  // Video metadata
  let videoTitle = "#video-title"
  let channelName = "#channel-name #text"
  let viewCount = "#metadata-line span:first-child"
  let uploadTime = "#metadata-line span:nth-child(2)"
  let thumbnail = "img#img"
  let duration = "span.ytd-thumbnail-overlay-time-status-renderer"

  // Shorts feed (different structure)
  let shortsContainer = "#contents.ytd-reel-shelf-renderer"
  let shortsItem = "ytd-reel-item-renderer"

  // Home feed tabs
  let feedTabs = "yt-chip-cloud-chip-renderer"

  // Search results (different from home feed)
  let searchContainer = "#contents.ytd-section-list-renderer"
  let searchItem = "ytd-video-renderer"
}

// Detect if we're on YouTube
let detectPlatform = (url: string): bool => {
  url->String.includes("youtube.com") || url->String.includes("youtu.be")
}

// Classify video topic based on title/channel (simple heuristic)
let classifyContent = (title: string, channel: string): array<Observer.ContentSignal.category> => {
  let titleLower = title->String.toLowerCase
  let channelLower = channel->String.toLowerCase

  let categories = []

  // Tech keywords
  if titleLower->String.includes("programming") ||
     titleLower->String.includes("coding") ||
     titleLower->String.includes("software") ||
     titleLower->String.includes("tech") ||
     channelLower->String.includes("developer") {
    categories->Array.push(Observer.ContentSignal.Tech)->ignore
  }

  // Politics keywords
  if titleLower->String.includes("election") ||
     titleLower->String.includes("government") ||
     titleLower->String.includes("policy") ||
     titleLower->String.includes("political") {
    categories->Array.push(Observer.ContentSignal.Politics)->ignore
  }

  // Art keywords
  if titleLower->String.includes("art") ||
     titleLower->String.includes("painting") ||
     titleLower->String.includes("design") ||
     titleLower->String.includes("creative") {
    categories->Array.push(Observer.ContentSignal.Art)->ignore
  }

  // Science keywords
  if titleLower->String.includes("science") ||
     titleLower->String.includes("physics") ||
     titleLower->String.includes("biology") ||
     titleLower->String.includes("research") {
    categories->Array.push(Observer.ContentSignal.Science)->ignore
  }

  // Entertainment keywords
  if titleLower->String.includes("funny") ||
     titleLower->String.includes("comedy") ||
     titleLower->String.includes("movie") ||
     titleLower->String.includes("review") {
    categories->Array.push(Observer.ContentSignal.Entertainment)->ignore
  }

  // Default to Unknown if no match
  if Array.length(categories) == 0 {
    categories->Array.push(Observer.ContentSignal.Unknown)->ignore
  }

  categories
}

// Extract signal from a single video element
let extractVideoSignal = (element: Dom.element): option<Observer.ContentSignal.t> => {
  // Extract title
  let titleOpt = Dom.elementQuerySelector(element, Selectors.videoTitle)
    ->Null.toOption
    ->Option.map(Dom.getTextSafe)

  // Extract channel name
  let channelOpt = Dom.elementQuerySelector(element, Selectors.channelName)
    ->Null.toOption
    ->Option.map(Dom.getTextSafe)

  // Extract video URL from title link
  let urlOpt = Dom.elementQuerySelector(element, Selectors.videoTitle)
    ->Null.toOption
    ->Option.flatMap(titleEl =>
      Dom.getAttributeSafe(titleEl, "href")
        ->String.trim
        ->Option.fromPredicate(url => url != "")
    )

  // Only create signal if we have at least a title and URL
  switch (titleOpt, urlOpt) {
  | (Some(title), Some(url)) => {
      let channel = channelOpt->Option.getOr("Unknown Channel")
      let categories = classifyContent(title, channel)

      Some({
        Observer.ContentSignal.url: url,
        text: Some(title),
        categories: categories,
        timestamp: Date.now(),
        platformSource: "youtube",
        confidence: if Array.length(categories) > 0 { 0.7 } else { 0.3 },
      })
    }
  | _ => None
  }
}

// Extract all signals from YouTube feed
let extractSignals = (document: Dom.document): array<Observer.ContentSignal.t> => {
  // Query all video items from the feed
  let videoNodeList = Dom.querySelectorAll(document, Selectors.videoItem)
  let videoElements = Dom.nodeListToArray(videoNodeList)

  // Extract signals from each video element
  videoElements
    ->Array.map(extractVideoSignal)
    ->Array.filterMap(x => x)  // Filter out None values
}

// Get current feed diversity
let analyzeFeedDiversity = (signals: array<Observer.ContentSignal.t>): float => {
  Observer.analyzeDiversity(signals)
}
