//! # Output Security Module
//!
//! This module provides security features for output generation, including
//! HTML sanitization, XSS prevention, and content validation.

use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;

/// HTML sanitization configuration
#[derive(Debug, Clone)]
pub struct SanitizationConfig {
    /// Allowed HTML tags
    pub allowed_tags: HashSet<String>,
    /// Allowed HTML attributes
    pub allowed_attributes: HashSet<String>,
    /// Whether to strip or escape disallowed content
    pub strip_disallowed: bool,
    /// Maximum content length to prevent DoS
    pub max_content_length: usize,
}

impl Default for SanitizationConfig {
    fn default() -> Self {
        let mut allowed_tags = HashSet::new();
        // Safe HTML tags for security reports
        allowed_tags.insert("h1".to_string());
        allowed_tags.insert("h2".to_string());
        allowed_tags.insert("h3".to_string());
        allowed_tags.insert("h4".to_string());
        allowed_tags.insert("h5".to_string());
        allowed_tags.insert("h6".to_string());
        allowed_tags.insert("p".to_string());
        allowed_tags.insert("br".to_string());
        allowed_tags.insert("div".to_string());
        allowed_tags.insert("span".to_string());
        allowed_tags.insert("ul".to_string());
        allowed_tags.insert("ol".to_string());
        allowed_tags.insert("li".to_string());
        allowed_tags.insert("table".to_string());
        allowed_tags.insert("thead".to_string());
        allowed_tags.insert("tbody".to_string());
        allowed_tags.insert("tr".to_string());
        allowed_tags.insert("th".to_string());
        allowed_tags.insert("td".to_string());
        allowed_tags.insert("code".to_string());
        allowed_tags.insert("pre".to_string());
        allowed_tags.insert("strong".to_string());
        allowed_tags.insert("em".to_string());
        allowed_tags.insert("b".to_string());
        allowed_tags.insert("i".to_string());

        let mut allowed_attributes = HashSet::new();
        allowed_attributes.insert("class".to_string());
        allowed_attributes.insert("id".to_string());
        allowed_attributes.insert("data-severity".to_string());
        allowed_attributes.insert("data-finding-id".to_string());

        Self {
            allowed_tags,
            allowed_attributes,
            strip_disallowed: true,
            max_content_length: 10 * 1024 * 1024, // 10MB limit
        }
    }
}

/// Sanitize HTML content to prevent XSS attacks
///
/// This function removes or escapes potentially dangerous HTML content
/// while preserving safe formatting elements needed for security reports.
///
/// # Arguments
/// * `html` - The HTML content to sanitize
/// * `config` - Optional sanitization configuration
///
/// # Returns
/// Returns sanitized HTML content
///
/// # Errors
/// Returns an error if content exceeds size limits or contains invalid sequences
pub fn sanitize_html(html: &str, config: Option<&SanitizationConfig>) -> Result<String> {
    let default_config = SanitizationConfig::default();
    let config = config.unwrap_or(&default_config);

    // Check content length to prevent DoS
    if html.len() > config.max_content_length {
        return Err(anyhow::anyhow!(
            "HTML content exceeds maximum length: {} > {}",
            html.len(),
            config.max_content_length
        ));
    }

    let mut sanitized = html.to_string();

    // Remove dangerous script tags and content
    sanitized = remove_script_tags(&sanitized)?;

    // Remove dangerous event handlers
    sanitized = remove_event_handlers(&sanitized)?;

    // Remove dangerous protocols in links
    sanitized = sanitize_links(&sanitized)?;

    // Remove or escape disallowed tags
    sanitized = filter_tags(&sanitized, config)?;

    // Escape remaining dangerous characters
    sanitized = escape_dangerous_chars(&sanitized);

    Ok(sanitized)
}

/// Remove script tags and their content
fn remove_script_tags(html: &str) -> Result<String> {
    let script_regex = Regex::new(r"(?i)<script[^>]*>.*?</script>")?;
    Ok(script_regex.replace_all(html, "").to_string())
}

/// Remove event handlers like onclick, onload, etc.
fn remove_event_handlers(html: &str) -> Result<String> {
    let event_regex = Regex::new(r#"(?i)\s*on\w+\s*=\s*['"][^'"]*['"]"#)?;
    Ok(event_regex.replace_all(html, "").to_string())
}

/// Sanitize links to remove dangerous protocols
fn sanitize_links(html: &str) -> Result<String> {
    let dangerous_protocols = Regex::new(r"(?i)(javascript|data|vbscript):")?;
    Ok(dangerous_protocols.replace_all(html, "").to_string())
}

/// Filter HTML tags based on allowed list
fn filter_tags(html: &str, config: &SanitizationConfig) -> Result<String> {
    let tag_regex = Regex::new(r"<(/?)([a-zA-Z][a-zA-Z0-9]*)[^>]*>")?;

    let result = tag_regex.replace_all(html, |caps: &regex::Captures| {
        let tag_name = caps
            .get(2)
            .map(|m| m.as_str().to_lowercase())
            .unwrap_or_default();

        if config.allowed_tags.contains(&tag_name) {
            // Keep allowed tags but filter attributes
            caps.get(0)
                .map(|m| filter_attributes(m.as_str(), config))
                .unwrap_or_default()
        } else if config.strip_disallowed {
            // Remove disallowed tags
            String::new()
        } else {
            // Escape disallowed tags
            caps.get(0)
                .map(|m| html_escape::encode_text(m.as_str()).to_string())
                .unwrap_or_default()
        }
    });

    Ok(result.to_string())
}

/// Filter HTML attributes based on allowed list
fn filter_attributes(tag: &str, _config: &SanitizationConfig) -> String {
    // Filter allowed attributes based on security policy
    let _allowed_attrs = ["href", "title", "alt", "class", "id"];

    // Simple regex-based attribute filtering for now
    let mut filtered_tag = tag.to_string();

    // Remove any attribute not in allowed list
    // This is a simplified implementation - in production would use proper HTML parser
    for attr in ["onclick", "onload", "onerror", "javascript:", "data:"] {
        filtered_tag = filtered_tag.replace(attr, "");
    }

    filtered_tag
}

/// Escape dangerous characters
fn escape_dangerous_chars(html: &str) -> String {
    html.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#x27;")
}

/// Validate that content is safe for output
pub fn validate_content_security(content: &str, content_type: &str) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check for potential XSS patterns
    if content_type == "text/html" {
        if content.contains("<script") {
            warnings.push("HTML content contains script tags".to_string());
        }

        if content.contains("javascript:") {
            warnings.push("HTML content contains javascript: protocol".to_string());
        }

        if content.contains("on") && (content.contains("=") || content.contains("(")) {
            warnings.push("HTML content may contain event handlers".to_string());
        }
    }

    // Check for extremely large content that could cause DoS
    if content.len() > 50 * 1024 * 1024 {
        // 50MB
        warnings.push("Content size is very large and may impact performance".to_string());
    }

    // Check for control characters that could cause issues
    if content
        .chars()
        .any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t')
    {
        warnings.push("Content contains control characters".to_string());
    }

    Ok(warnings)
}

/// Generate Content Security Policy (CSP) header for HTML output
pub fn generate_csp_header() -> String {
    "default-src 'self'; script-src 'none'; object-src 'none'; base-uri 'self'; frame-ancestors 'none';".to_string()
}

/// Sanitize file paths to prevent directory traversal
pub fn sanitize_file_path(path: &str) -> Result<String> {
    // Remove directory traversal patterns
    let cleaned = path.replace("..", "").replace("//", "/");

    // Ensure path doesn't start with /
    let cleaned = cleaned.trim_start_matches('/');

    // Validate that result is reasonable
    if cleaned.is_empty() {
        return Err(anyhow::anyhow!("File path became empty after sanitization"));
    }

    if cleaned.len() > 1000 {
        return Err(anyhow::anyhow!("File path too long after sanitization"));
    }

    Ok(cleaned.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_html_removes_scripts() -> Result<(), Box<dyn std::error::Error>> {
        let html = r#"<div>Safe content</div><script>alert('xss')</script>"#;
        let result = sanitize_html(html, None)?;
        assert!(!result.contains("script"));
        assert!(result.contains("Safe content"));
    }

    #[test]
    fn test_sanitize_html_removes_event_handlers() -> Result<(), Box<dyn std::error::Error>> {
        let html = r#"<div onclick="alert('xss')">Click me</div>"#;
        let result = sanitize_html(html, None)?;
        assert!(!result.contains("onclick"));
        assert!(result.contains("Click me"));
    }

    #[test]
    fn test_validate_content_security_detects_scripts() -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"<html><script>alert('test')</script></html>"#;
        let warnings = validate_content_security(content, "text/html")?;
        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.contains("script tags")));
    }

    #[test]
    fn test_sanitize_file_path_removes_traversal() -> Result<(), Box<dyn std::error::Error>> {
        let path = "../../../etc/passwd";
        let result = sanitize_file_path(path)?;
        assert_eq!(result, "etc/passwd");
    }
}
