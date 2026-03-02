// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Rule definitions and evaluation logic

use super::{Action, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: Operator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    Equals,
    Contains,
    GreaterThan,
    LessThan,
    Matches, // Regex
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub probability: f64, // 0.0 - 1.0
    pub enabled: bool,
}

impl Rule {
    /// Evaluate rule against context, return actions if conditions match
    pub fn evaluate(&self, context: &Context) -> Option<Vec<Action>> {
        if !self.enabled {
            return None;
        }

        // Check if all conditions are satisfied
        let all_conditions_met = self
            .conditions
            .iter()
            .all(|condition| self.evaluate_condition(condition, context));

        if !all_conditions_met {
            return None;
        }

        // Check probability
        if rand::random::<f64>() > self.probability {
            return None;
        }

        Some(self.actions.clone())
    }

    fn evaluate_condition(&self, condition: &Condition, context: &Context) -> bool {
        let context_value = match condition.field.as_str() {
            "platform" => serde_json::Value::String(context.platform.clone()),
            "content_type" => serde_json::Value::String(context.content_type.clone()),
            "scroll_depth" => serde_json::Value::Number(context.scroll_depth.into()),
            "session_duration" => serde_json::Value::Number(context.session_duration.into()),
            _ => return false,
        };

        match &condition.operator {
            Operator::Equals => context_value == condition.value,
            Operator::Contains => {
                if let (Some(ctx_str), Some(val_str)) =
                    (context_value.as_str(), condition.value.as_str()) {
                    ctx_str.contains(val_str)
                } else {
                    false
                }
            }
            Operator::GreaterThan => {
                if let (Some(ctx_num), Some(val_num)) =
                    (context_value.as_f64(), condition.value.as_f64()) {
                    ctx_num > val_num
                } else {
                    false
                }
            }
            Operator::LessThan => {
                if let (Some(ctx_num), Some(val_num)) =
                    (context_value.as_f64(), condition.value.as_f64()) {
                    ctx_num < val_num
                } else {
                    false
                }
            }
            Operator::Matches => {
                // Simplified - would use regex crate in production
                false
            }
        }
    }

    /// Generate human-readable narrative for this rule
    pub fn narrate(&self) -> String {
        format!("{}: {}", self.name, self.description)
    }

    // Predefined rule constructors

    pub fn noise_injection(probability: f64) -> Self {
        Rule {
            id: "noise-injection".to_string(),
            name: "Noise Injection".to_string(),
            description: "Occasionally click content outside your inferred profile".to_string(),
            conditions: vec![],
            actions: vec![Action::InjectNoise { count: 2 }],
            probability,
            enabled: true,
        }
    }

    pub fn profile_dilution(probability: f64) -> Self {
        Rule {
            id: "profile-dilution".to_string(),
            name: "Profile Dilution".to_string(),
            description: "Open diverse background tabs when viewing niche content".to_string(),
            conditions: vec![],
            actions: vec![Action::OpenBackgroundTabs {
                urls: vec![
                    "https://en.wikipedia.org/wiki/Special:Random".to_string(),
                    "https://news.ycombinator.com".to_string(),
                ],
            }],
            probability,
            enabled: true,
        }
    }

    pub fn engagement_disruption(scroll_limit: u32) -> Self {
        Rule {
            id: "engagement-disruption".to_string(),
            name: "Engagement Disruption".to_string(),
            description: format!("Suggest break after {} scroll events", scroll_limit),
            conditions: vec![Condition {
                field: "scroll_depth".to_string(),
                operator: Operator::GreaterThan,
                value: serde_json::Value::Number(scroll_limit.into()),
            }],
            actions: vec![Action::SuggestBreak],
            probability: 1.0,
            enabled: true,
        }
    }
}
