use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TagsError {
    #[error("Tag cannot be empty")]
    EmptyTag,
    #[error("Tag exceeds maximum length of 50 characters")]
    TagTooLong,
}

/// Maximum allowed length for a single tag
const MAX_TAG_LENGTH: usize = 50;

/// Value object representing station or network tags.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tags(pub Vec<String>);

impl Tags {
    /// Creates a new Tags VO with normalization and validation.
    pub fn new(tags: Vec<String>) -> Result<Self, TagsError> {
        let mut processed = Vec::new();
        let mut seen = HashSet::new();

        for tag in tags {
            let normalized = tag.trim().to_string();

            if normalized.is_empty() {
                return Err(TagsError::EmptyTag);
            }
            if normalized.len() > MAX_TAG_LENGTH {
                return Err(TagsError::TagTooLong);
            }

            if seen.insert(normalized.clone()) {
                processed.push(normalized);
            }
        }

        Ok(Self(processed))
    }

    pub fn inner(&self) -> &Vec<String> {
        &self.0
    }

    /// Immutable add
    pub fn with_tag(&self, tag: String) -> Result<Self, TagsError> {
        let mut new_tags = self.0.clone();
        new_tags.push(tag);
        Tags::new(new_tags)
    }

    /// Immutable remove
    pub fn without_tag(&self, tag: &str) -> Self {
        let filtered = self
            .0
            .iter()
            .cloned()
            .filter(|t| t != tag)
            .collect::<Vec<_>>();
        Self(filtered)
    }
}
