use crate::domain::error::DomainError;

pub const MAX_PAYLOAD_BYTES: usize = 512 * 1024; // 512 KB limit

pub const SXNNYSIDE_BANNER: &str = r#"
 ╔═══════════════════════════════════════════════════════════════╗
 ║  ██████╗ ███████╗██╗   ██╗ ██████╗██╗  ██╗ ██████╗            ║
 ║  ██╔══██╗██╔════╝╚██╗ ██╔╝██╔════╝██║  ██║██╔═══██╗           ║
 ║  ██████╔╝███████╗ ╚████╔╝ ██║     ███████║██║   ██║           ║
 ║  ██╔═══╝ ╚════██║  ╚██╔╝  ██║     ██╔══██║██║   ██║           ║
 ║  ██║     ███████║   ██║   ╚██████╗██║  ██║╚██████╔╝           ║
 ║  ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝ ╚═════╝            ║
 ║   ██████╗ ██╗   ██╗██╗███╗   ██╗███████╗                      ║
 ║  ██╔═══██╗██║   ██║██║████╗  ██║██╔════╝                      ║
 ║  ██║   ██║██║   ██║██║██╔██╗ ██║█████╗                        ║
 ║  ██║▄▄ ██║██║   ██║██║██║╚██╗██║██╔══╝                        ║
 ║  ╚██████╔╝╚██████╔╝██║██║ ╚████║███████╗                      ║
 ║   ╚══▀▀═╝  ╚═════╝ ╚═╝╚═╝  ╚═══╝╚══════╝                      ║
 ║                                                               ║
 ║  Universal Quine Generator                  Sxnnyside Project ║
 ╚═══════════════════════════════════════════════════════════════╝
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PayloadMode {
    None,
    Text(String),
    Banner,
}

#[derive(Debug, Clone)]
pub struct Payload {
    pub content: String,
    pub is_banner: bool,
}

impl Payload {
    pub fn try_from_mode(mode: &PayloadMode) -> Result<Option<Self>, DomainError> {
        match mode {
            PayloadMode::None => Ok(None),
            PayloadMode::Text(text) => {
                let trimmed = text.trim();
                if trimmed.is_empty() {
                    Ok(None)
                } else if trimmed.len() > MAX_PAYLOAD_BYTES {
                    Err(DomainError::PayloadTooLarge {
                        max_bytes: MAX_PAYLOAD_BYTES,
                        actual_bytes: trimmed.len(),
                    })
                } else {
                    Ok(Some(Payload {
                        content: trimmed.to_string(),
                        is_banner: false,
                    }))
                }
            }
            PayloadMode::Banner => Ok(Some(Payload {
                content: SXNNYSIDE_BANNER.trim().to_string(),
                is_banner: true,
            })),
        }
    }

    pub fn from_mode(mode: &PayloadMode) -> Option<Self> {
        Self::try_from_mode(mode).ok().flatten()
    }

    /// Format payload as comment lines for a specific comment prefix
    pub fn format_as_comment(&self, comment_prefix: &str) -> String {
        if comment_prefix.is_empty() {
            return String::new();
        }

        let mut out = String::new();
        for line in self.content.lines() {
            out.push_str(comment_prefix);
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}
