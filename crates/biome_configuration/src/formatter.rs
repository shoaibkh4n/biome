use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{IndentStyle, LineEnding, LineWidth};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Generic options applied to all files
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct FormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    #[partial(bpaf(hide))]
    pub enabled: bool,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    #[partial(bpaf(hide))]
    pub format_with_errors: bool,

    /// The indent style.
    #[partial(bpaf(long("indent-style"), argument("tab|space"), optional))]
    pub indent_style: PlainIndentStyle,

    /// The size of the indentation, 2 by default (deprecated, use `indent-width`)
    #[partial(bpaf(long("indent-size"), argument("NUMBER"), optional))]
    #[partial(deserializable(deprecated(use_instead = "formatter.indentWidth")))]
    pub indent_size: u8,

    /// The size of the indentation, 2 by default
    #[partial(bpaf(long("indent-width"), argument("NUMBER"), optional))]
    pub indent_width: u8,

    /// The type of line ending.
    #[partial(bpaf(long("line-ending"), argument("lf|crlf|cr"), optional))]
    pub line_ending: LineEnding,

    /// What's the max width of a line. Defaults to 80.
    #[partial(bpaf(long("line-width"), argument("NUMBER"), optional))]
    #[partial(serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    ))]
    pub line_width: LineWidth,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub ignore: StringSet,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub include: StringSet,
}

impl PartialFormatterConfiguration {
    pub const fn is_disabled(&self) -> bool {
        matches!(self.enabled, Some(false))
    }
}

impl Default for FormatterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            format_with_errors: false,
            indent_size: 2,
            indent_width: 2,
            indent_style: PlainIndentStyle::default(),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
            ignore: Default::default(),
            include: Default::default(),
        }
    }
}

/// Required by [Bpaf].
impl FromStr for FormatterConfiguration {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

impl From<PlainIndentStyle> for IndentStyle {
    fn from(value: PlainIndentStyle) -> Self {
        match value {
            PlainIndentStyle::Tab => IndentStyle::Tab,
            PlainIndentStyle::Space => IndentStyle::Space,
        }
    }
}

pub fn deserialize_line_width<'de, D>(deserializer: D) -> Result<Option<LineWidth>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: u16 = Deserialize::deserialize(deserializer)?;
    let line_width = LineWidth::try_from(value).map_err(serde::de::Error::custom)?;
    Ok(Some(line_width))
}

pub fn serialize_line_width<S>(line_width: &Option<LineWidth>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    s.serialize_u16(line_width.unwrap_or_default().get())
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum PlainIndentStyle {
    /// Tab
    #[default]
    Tab,
    /// Space
    Space,
}

impl FromStr for PlainIndentStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tab" => Ok(PlainIndentStyle::Tab),
            "space" => Ok(PlainIndentStyle::Space),
            _ => Err("Unsupported value for this option"),
        }
    }
}