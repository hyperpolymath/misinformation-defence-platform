// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Integration test for WASM rule engine

open RuleEngine

// Test creating the engine and adding a rule
let testRuleEngine = async () => {
  Console.log("Testing WASM Rule Engine...")

  // Create engine
  let engine = await create()
  Console.log("✓ Engine created")

  // Add a noise injection rule
  let ruleJson = `{
    "id": "test-noise",
    "name": "Test Noise Injection",
    "description": "Test rule for integration",
    "conditions": [],
    "actions": [{"type": "InjectNoise", "payload": {"count": 2}}],
    "probability": 1.0,
    "enabled": true
  }`

  let result = await addRuleSafe(engine, ruleJson)
  switch result {
  | Ok() => Console.log("✓ Rule added successfully")
  | Error(err) => Console.error("✗ Failed to add rule: " ++ err)
  }

  // Get narrative
  let narrative = narrateRules(engine)
  Console.log("✓ Rule narrative: " ++ narrative)

  // Evaluate against context
  let contextJson = `{
    "platform": "youtube",
    "content_type": "video",
    "scroll_depth": 10,
    "session_duration": 120,
    "recent_categories": ["tech"],
    "timestamp": 0
  }`

  let evalResult = await evaluateSafe(engine, contextJson)
  switch evalResult {
  | Ok(actions) => Console.log("✓ Evaluation successful: " ++ actions)
  | Error(err) => Console.error("✗ Evaluation failed: " ++ err)
  }

  Console.log("WASM integration test complete!")
}
