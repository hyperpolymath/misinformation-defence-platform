// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Actions: What the rule engine instructs the extension to do

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Action {
    InjectNoise { count: u32 },
    OpenBackgroundTabs { urls: Vec<String> },
    SuggestBreak,
    ClickOffTopic,
    ScrollLimit { max_items: u32 },
    ApplyLens { lens_name: String },
    Log { message: String },
}

impl Action {
    /// Generate human-readable description
    pub fn narrate(&self) -> String {
        match self {
            Action::InjectNoise { count } => {
                format!("Inject {} off-profile clicks", count)
            }
            Action::OpenBackgroundTabs { urls } => {
                format!("Open {} background tabs for profile dilution", urls.len())
            }
            Action::SuggestBreak => "Suggest taking a break".to_string(),
            Action::ClickOffTopic => "Click off-topic content".to_string(),
            Action::ScrollLimit { max_items } => {
                format!("Limit scroll to {} items", max_items)
            }
            Action::ApplyLens { lens_name } => {
                format!("Apply lens: {}", lens_name)
            }
            Action::Log { message } => message.clone(),
        }
    }
}
