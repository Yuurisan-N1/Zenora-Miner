use crate::support::banner::print_banner;
use std::path::PathBuf;

pub fn project_root() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn clean_text(value: &str, fallback: &str) -> String {
    let mut text = String::with_capacity(value.len());
    for ch in value.chars() {
        if "[]|#!@$%^&*()-".contains(ch) {
            text.push(' ');
        } else {
            text.push(ch);
        }
    }
    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if cleaned.is_empty() {
        fallback.to_string()
    } else {
        cleaned
    }
}

pub fn show_banner() {
    print_banner();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_text_strips_the_banned_symbols() {
        assert_eq!(clean_text("hello|world", "x"), "hello world");
        assert_eq!(clean_text("", "Unknown"), "Unknown");
    }
}
