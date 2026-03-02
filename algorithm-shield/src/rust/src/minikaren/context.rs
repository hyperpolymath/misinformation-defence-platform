// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 hyperpolymath
// Part of Algorithm Shield - https://github.com/hyperpolymath/algorithm-shield
// Context: The state against which rules are evaluated

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub platform: String,
    pub content_type: String,
    pub scroll_depth: u32,
    pub session_duration: u32, // seconds
    pub recent_categories: Vec<String>,
    pub timestamp: u64,
}

impl Context {
    pub fn new(platform: &str, content_type: &str) -> Self {
        Context {
            platform: platform.to_string(),
            content_type: content_type.to_string(),
            scroll_depth: 0,
            session_duration: 0,
            recent_categories: Vec::new(),
            timestamp: 0,
        }
    }

    pub fn with_scroll_depth(mut self, depth: u32) -> Self {
        self.scroll_depth = depth;
        self
    }

    pub fn with_session_duration(mut self, duration: u32) -> Self {
        self.session_duration = duration;
        self
    }

    pub fn with_categories(mut self, categories: Vec<String>) -> Self {
        self.recent_categories = categories;
        self
    }
}
