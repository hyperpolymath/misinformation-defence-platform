// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// WASM entry point for Algorithm Shield rule engine
//
// v1.0: Uses serde_json (WASM-compatible)
// v2.0: Will use proven library via Ephapax hot paths (Idris2-verified, Zig FFI)
//       - Ephapax bindings ready: /var/mnt/eclipse/repos/proven/bindings/ephapax
//       - Zig FFI layer ready: /var/mnt/eclipse/repos/proven/ffi/zig
//       - See: docs/PROVEN-EPHAPAX-INTEGRATION.adoc

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

mod minikaren;

#[wasm_bindgen]
pub struct RuleEngine {
    rules: Vec<minikaren::Rule>,
}

#[wasm_bindgen]
impl RuleEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        RuleEngine { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule_json: &str) -> Result<(), JsValue> {
        // v1.0: serde_json (WASM-compatible)
        // v2.0: Will use proven::SafeJson via Ephapax hot paths
        let rule: minikaren::Rule = serde_json::from_str(rule_json)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;
        self.rules.push(rule);
        Ok(())
    }

    pub fn evaluate(&self, context_json: &str) -> Result<String, JsValue> {
        // v1.0: serde_json (WASM-compatible)
        // v2.0: Will use proven::SafeJson via Ephapax hot paths
        let context: minikaren::Context = serde_json::from_str(context_json)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

        let actions = minikaren::evaluate_rules(&self.rules, &context);

        serde_json::to_string(&actions)
            .map_err(|e| JsValue::from_str(&format!("Serialize error: {}", e)))
    }

    pub fn narrate_rules(&self) -> String {
        self.rules
            .iter()
            .map(|r| format!("• {}", r.narrate()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
