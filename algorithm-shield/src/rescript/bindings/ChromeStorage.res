// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// ReScript bindings for chrome.storage.local API

module Local = {
  type storageArea

  // Get chrome.storage.local
  @val @scope(("chrome", "storage"))
  external local: storageArea = "local"

  // Get values from storage
  @send
  external get: (storageArea, array<string>) => promise<{..}> = "get"

  // Set values in storage
  @send
  external set: (storageArea, {..}) => promise<unit> = "set"

  // Remove values from storage
  @send
  external remove: (storageArea, array<string>) => promise<unit> = "remove"

  // Clear all values
  @send
  external clear: storageArea => promise<unit> = "clear"

  // Helper: get a single key
  let getSingle = async (key: string) => {
    let result = await get(local, [key])
    result
  }

  // Helper: set a single key-value pair
  let setSingle = async (key: string, value: 'a) => {
    let obj = Obj.magic(Dict.fromArray([(key, value)]))
    await set(local, obj)
  }
}
