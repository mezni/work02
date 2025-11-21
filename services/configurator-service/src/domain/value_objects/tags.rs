use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TagsError {
    #[error("Too many tags: {0} (max 10)")]
    TooManyTags(usize),
    #[error("Tag too long: '{0}' (max 50 characters)")]
    TagTooLong(String),
    #[error("Tag contains invalid characters: '{0}' (only alphanumeric and hyphens allowed)")]
    InvalidCharacters(String),
    #[error("Duplicate tag: '{0}'")]
    DuplicateTag(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tags(Vec<String>);

impl Tags {
    const MAX_TAGS: usize = 10;
    const MAX_TAG_LENGTH: usize = 50;

    pub fn new(tags: Vec<String>) -> Result<Self, TagsError> {
        if tags.len() > Self::MAX_TAGS {
            return Err(TagsError::TooManyTags(tags.len()));
        }

        let mut seen = std::collections::HashSet::new();
        for tag in &tags {
            // Check length
            if tag.len() > Self::MAX_TAG_LENGTH {
                return Err(TagsError::TagTooLong(tag.clone()));
            }

            // Check characters (alphanumeric and hyphens only)
            let tag_regex = regex::Regex::new(r"^[a-zA-Z0-9\-]+$").unwrap();
            if !tag_regex.is_match(tag) {
                return Err(TagsError::InvalidCharacters(tag.clone()));
            }

            // Check for duplicates (case insensitive)
            let lower_tag = tag.to_lowercase();
            if seen.contains(&lower_tag) {
                return Err(TagsError::DuplicateTag(tag.clone()));
            }
            seen.insert(lower_tag);
        }

        Ok(Tags(tags))
    }

    pub fn from_comma_separated(tags_str: &str) -> Result<Self, TagsError> {
        let tags: Vec<String> = tags_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Self::new(tags)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.0.iter()
    }

    pub fn contains(&self, tag: &str) -> bool {
        self.0
            .iter()
            .any(|t| t.to_lowercase() == tag.to_lowercase())
    }

    pub fn add_tag(&mut self, tag: String) -> Result<(), TagsError> {
        let mut new_tags = self.0.clone();
        new_tags.push(tag);
        *self = Self::new(new_tags)?;
        Ok(())
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.0.retain(|t| t.to_lowercase() != tag.to_lowercase());
    }
}

impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

impl FromStr for Tags {
    type Err = TagsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_comma_separated(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tags() {
        let valid_tags = vec![
            vec!["tag1".to_string(), "tag2".to_string()],
            vec![
                "rust".to_string(),
                "web-development".to_string(),
                "api".to_string(),
            ],
            vec!["123".to_string(), "test-123".to_string()],
        ];

        for tags in valid_tags {
            assert!(Tags::new(tags).is_ok());
        }
    }

    #[test]
    fn test_invalid_tags() {
        // Too many tags
        let too_many = (1..=11).map(|i| format!("tag{}", i)).collect();
        assert!(Tags::new(too_many).is_err());

        // Tag too long
        let long_tag = vec!["a".repeat(51)];
        assert!(Tags::new(long_tag).is_err());

        // Invalid characters
        let invalid_chars = vec!["tag with spaces".to_string()];
        assert!(Tags::new(invalid_chars).is_err());

        // Duplicate tags
        let duplicates = vec!["tag1".to_string(), "TAG1".to_string()];
        assert!(Tags::new(duplicates).is_err());
    }

    #[test]
    fn test_from_comma_separated() {
        let tags = Tags::from_comma_separated("rust, web, api").unwrap();
        assert_eq!(tags.len(), 3);
        assert!(tags.contains("rust"));
        assert!(tags.contains("web"));
        assert!(tags.contains("api"));
    }

    #[test]
    fn test_add_remove_tags() {
        let mut tags = Tags::new(vec!["tag1".to_string(), "tag2".to_string()]).unwrap();

        tags.add_tag("tag3".to_string()).unwrap();
        assert_eq!(tags.len(), 3);
        assert!(tags.contains("tag3"));

        tags.remove_tag("tag1");
        assert_eq!(tags.len(), 2);
        assert!(!tags.contains("tag1"));
    }

    #[test]
    fn test_display() {
        let tags = Tags::new(vec!["rust".to_string(), "web".to_string()]).unwrap();
        assert_eq!(tags.to_string(), "rust, web");
    }
}
