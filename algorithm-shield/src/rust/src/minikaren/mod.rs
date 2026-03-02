// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Minikaren-style logic programming engine for Algorithm Shield rules

use serde::{Deserialize, Serialize};

mod rules;
mod context;
mod actions;

pub use rules::Rule;
pub use context::Context;
pub use actions::Action;

/// Evaluate all rules against a context and return matching actions
pub fn evaluate_rules(rules: &[Rule], context: &Context) -> Vec<Action> {
    rules
        .iter()
        .filter_map(|rule| rule.evaluate(context))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_evaluation() {
        let rule = Rule::noise_injection(0.3);
        let context = Context::new("youtube.com", "tech");

        let actions = rule.evaluate(&context);
        assert!(actions.is_some());
    }
}
