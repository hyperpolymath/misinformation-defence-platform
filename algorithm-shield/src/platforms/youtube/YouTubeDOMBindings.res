// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield

// Low-level DOM bindings for YouTube

// DOM types
type element
type nodeList
type document

@val external document: document = "document"

// Query selectors
@send external querySelector: (document, string) => Null.t<element> = "querySelector"
@send external querySelectorAll: (document, string) => nodeList = "querySelectorAll"
@send external elementQuerySelector: (element, string) => Null.t<element> = "querySelector"

// NodeList operations
@send external nodeListItem: (nodeList, int) => Null.t<element> = "item"
@get external nodeListLength: nodeList => int = "length"

// Element properties
@get external textContent: element => Null.t<string> = "textContent"
@get external getAttribute: (element, string) => Null.t<string> = "getAttribute"

// Helper to convert NodeList to array
let nodeListToArray = (nodeList: nodeList): array<element> => {
  let length = nodeListLength(nodeList)
  let result = []

  for i in 0 to length - 1 {
    switch nodeListItem(nodeList, i)->Null.toOption {
    | Some(el) => result->Array.push(el)->ignore
    | None => ()
    }
  }

  result
}

// Helper to safely get text content
let getTextSafe = (element: element): string => {
  textContent(element)->Null.toOption->Option.getOr("")
}

// Helper to safely get attribute
let getAttributeSafe = (element: element, attr: string): string => {
  getAttribute(element, attr)->Null.toOption->Option.getOr("")
}

// MutationObserver for feed updates
type mutationObserver
type mutationObserverInit = {
  childList: bool,
  subtree: bool,
}

@new external createMutationObserver: ((array<'a>, mutationObserver) => unit) => mutationObserver = "MutationObserver"
@send external observe: (mutationObserver, element, mutationObserverInit) => unit = "observe"
@send external disconnect: mutationObserver => unit = "disconnect"

// Set up observer for feed changes
let observeFeedChanges = (callback: unit => unit): option<mutationObserver> => {
  let containerOpt = querySelector(document, YouTubeAdapter.Selectors.feedContainer)->Null.toOption

  switch containerOpt {
  | None => None
  | Some(container) => {
      let observer = createMutationObserver((_mutations, _observer) => {
        callback()
      })

      observe(observer, container, {
        childList: true,
        subtree: true,
      })

      Some(observer)
    }
  }
}
