// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// ReScript bindings for the Rust/WASM rule engine

// The WASM module type
type t

// Initialize the rule engine
@module("../../../pkg/algorithm_shield_engine.js")
external make: unit => t = "RuleEngine"

// Add a rule (takes JSON string)
@send
external addRule: (t, string) => promise<unit> = "add_rule"

// Evaluate rules against context (takes JSON string, returns JSON string)
@send
external evaluate: (t, string) => promise<string> = "evaluate"

// Get human-readable narrative of all rules
@send
external narrateRules: t => string = "narrate_rules"

// Helper to create the engine asynchronously
let create = async () => {
  make()
}

// Helper to add a rule with error handling
let addRuleSafe = async (engine: t, ruleJson: string) => {
  try {
    await addRule(engine, ruleJson)
    Ok()
  } catch {
  | Exn.Error(err) => Error(Exn.message(err)->Option.getOr("Unknown error adding rule"))
  }
}

// Helper to evaluate with error handling
let evaluateSafe = async (engine: t, contextJson: string) => {
  try {
    let result = await evaluate(engine, contextJson)
    Ok(result)
  } catch {
  | Exn.Error(err) => Error(Exn.message(err)->Option.getOr("Unknown error evaluating rules"))
  }
}
