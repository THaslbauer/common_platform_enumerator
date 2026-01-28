use crate::component::Component;
use crate::cpe::{CpeType, Language};
use crate::wfn::Wfn;
use std::fmt;

pub struct FormattedString<'a>(Wfn<'a>);

trait FormattedDisplay {
    fn special_fmt(&self) -> &str;
}

impl FormattedDisplay for Component<'_> {
    fn special_fmt(&self) -> &str {
        match self {
            Self::Any => "*",
            Self::NotApplicable => "-",
            Self::Value(val) => val,
        }
    }
}

impl FormattedDisplay for Language {
    fn special_fmt(&self) -> &str {
        match self {
            Self::Any => "*",
            Self::Language(tag) => tag.as_str(),
        }
    }
}

impl<'a> fmt::Display for FormattedString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let wfn = &self.0;
        write!(f, "cpe:2.3:")?;
        match wfn.part {
            CpeType::Any => write!(f, "*")?,
            CpeType::Hardware => write!(f, "h")?,
            CpeType::OperatingSystem => write!(f, "o")?,
            CpeType::Application => write!(f, "a")?,
            CpeType::Empty => write!(f, "-")?,
        };
        write!(f, ":{}", wfn.vendor.special_fmt())?;
        write!(f, ":{}", wfn.product.special_fmt())?;
        write!(f, ":{}", wfn.version.special_fmt())?;
        write!(f, ":{}", wfn.update.special_fmt())?;
        write!(f, ":{}", wfn.edition.special_fmt())?;
        write!(f, ":{}", wfn.language.special_fmt())?;
        write!(f, ":{}", wfn.sw_edition.special_fmt())?;
        write!(f, ":{}", wfn.target_sw.special_fmt())?;
        write!(f, ":{}", wfn.other.special_fmt())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::wfn;

    #[test]
    fn test_display() {
        let wfn = wfn! {part: "a", vendor: "rust", product: "cargo", version: r"1\.0"}.unwrap();
        let fmt_str = FormattedString(wfn);
        assert_eq!("cpe:2.3:a:rust:cargo:1.0:*:*:*:*:*:*", fmt_str.to_string())
    }
}
