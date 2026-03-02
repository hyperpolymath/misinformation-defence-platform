// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// ReScript bindings for chrome.tabs API

type tab = {
  id: option<int>,
  url: option<string>,
  title: option<string>,
  active: bool,
  windowId: int,
}

type createProperties = {
  url: option<string>,
  active: option<bool>,
  windowId: option<int>,
}

// Query options
type queryInfo = {
  active: option<bool>,
  currentWindow: option<bool>,
}

module Tabs = {
  // Create a new tab
  @val @scope(("chrome", "tabs"))
  external create: createProperties => promise<tab> = "create"

  // Query tabs
  @val @scope(("chrome", "tabs"))
  external query: queryInfo => promise<array<tab>> = "query"

  // Close a tab
  @val @scope(("chrome", "tabs"))
  external remove: int => promise<unit> = "remove"

  // Update a tab
  @val @scope(("chrome", "tabs"))
  external update: (int, {"url": option<string>}) => promise<tab> = "update"

  // Helper: create tab with URL
  let createWithUrl = async (url: string, active: bool) => {
    let props = {
      url: Some(url),
      active: Some(active),
      windowId: None,
    }
    await create(props)
  }

  // Helper: get current active tab
  let getActive = async () => {
    let queryInfo = {
      active: Some(true),
      currentWindow: Some(true),
    }
    let tabs = await query(queryInfo)
    Array.get(tabs, 0)
  }
}
