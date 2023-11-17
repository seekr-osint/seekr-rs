use serde::Deserialize;
use std::fmt;

// TODO: better impl
macro_rules! display_option {
    ($format_string:expr,$format_string2:expr,$v:expr) => {
        format!(
            "{}",
                match $v {
                    Option::Some(v) => format!($format_string2, v),
                    Option::None => "".to_string(),
                }
        )
    };
    ($format_string:expr,$format_string2:expr,$($v:expr),+) => {
        format!(
            $format_string,
            $(display_option!("{}",$format_string2,$v)),+
        )
    };
}

#[derive(Debug, Clone, Deserialize)]
pub enum Title {
    Dr,
    Prof,
    Other(String),
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dr => write!(f, "Dr."),
            Self::Prof => write!(f, "Prof."),
            Self::Other(text) => write!(f, "{}", text),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Name {
    pub title: Option<Title>,
    pub first: String,
    pub middle: Option<String>,
    pub last: Option<String>,
}

impl Name {
    pub fn new(
        title: Option<Title>,
        first: impl Into<String>,
        middle: Option<String>,
        last: Option<String>,
    ) -> Self {
        Self {
            title,
            first: first.into(),
            middle,
            last,
        }
    }
}

/// # Examples
/// ```rust
/// # use seekr::name;
/// # fn main() {
/// let foo = name::Name::new(Some(name::Title::Dr), "tom", None,None);
/// let foo2 = format!("{}", foo);
/// assert_eq!("Dr. tom ",foo2);
/// # }
///
/// ```
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            display_option!(
                "{}{}{}",
                "{} ",
                &self.title,
                Some(&self.first),
                &self.middle
            )
        )
    }
}
