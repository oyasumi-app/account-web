use std::fmt::Display;

/// The options for colors provided by Bootstrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

impl Color {
    /// Get the name of the color.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Danger => "danger",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Primary
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// The options for sizes provided by Bootstrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Size {
    Small,
    Default,
    Large,
}

impl Size {
    /// Get the name of the size.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "",
            Self::Large => "lg",
        }
    }

    /// Get the class name of the size.
    /// For example, `btn` -> `btn-sm`.
    pub fn class(&self, class: &str) -> String {
        if self == &Self::Default {
            class.to_string()
        } else {
            format!("{}-{}", class, self.name())
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Default
    }
}
