use crate::options::parser::MatchedFlags;
use crate::options::vars::{self, Vars};
use crate::options::{flags, NumberSource, OptionsError};

use crate::output::file_name::{Classify, EmbedHyperlinks, Options, QuoteStyle, ShowIcons};

impl Options {
    pub fn deduce<V: Vars>(
        matches: &MatchedFlags<'_>,
        vars: &V,
        is_a_tty: bool,
    ) -> Result<Self, OptionsError> {
        let classify = Classify::deduce(matches)?;
        let show_icons = ShowIcons::deduce(matches, vars)?;

        let quote_style = QuoteStyle::deduce(matches)?;
        let embed_hyperlinks = EmbedHyperlinks::deduce(matches)?;

        Ok(Self {
            classify,
            show_icons,
            quote_style,
            embed_hyperlinks,
            is_a_tty,
        })
    }
}

impl Classify {
    fn deduce(matches: &MatchedFlags<'_>) -> Result<Self, OptionsError> {
        let flagged = matches.has(&flags::CLASSIFY)?;

        if flagged {
            Ok(Self::AddFileIndicators)
        } else {
            Ok(Self::JustFilenames)
        }
    }
}

impl ShowIcons {
    pub fn deduce<V: Vars>(matches: &MatchedFlags<'_>, vars: &V) -> Result<Self, OptionsError> {
        enum AlwaysOrAuto {
            Always,
            Automatic,
        }

        let mode_opt = matches.get(&flags::ICONS)?;
        if !matches.has(&flags::ICONS)? && mode_opt.is_none() {
            return Ok(Self::Never);
        }

        let mode = match mode_opt {
            Some(word) => match word.to_str() {
                Some("always") => AlwaysOrAuto::Always,
                Some("auto" | "automatic") => AlwaysOrAuto::Automatic,
                Some("never") => return Ok(Self::Never),
                None => AlwaysOrAuto::Automatic,
                _ => return Err(OptionsError::BadArgument(&flags::ICONS, word.into())),
            },
            None => AlwaysOrAuto::Automatic,
        };

        let width = if let Some(columns) = vars
            .get_with_fallback(vars::EXA_ICON_SPACING, vars::EZA_ICON_SPACING)
            .and_then(|s| s.into_string().ok())
        {
            match columns.parse() {
                Ok(width) => width,
                Err(e) => {
                    let source = NumberSource::Env(
                        vars.source(vars::EXA_ICON_SPACING, vars::EZA_ICON_SPACING)
                            .unwrap(),
                    );
                    return Err(OptionsError::FailedParse(columns, source, e));
                }
            }
        } else {
            1
        };

        match mode {
            AlwaysOrAuto::Always => Ok(Self::Always(width)),
            AlwaysOrAuto::Automatic => Ok(Self::Automatic(width)),
        }
    }
}

impl QuoteStyle {
    pub fn deduce(matches: &MatchedFlags<'_>) -> Result<Self, OptionsError> {
        if let Ok(env) = std::env::var("EZA_QUOTE_STYLE") {
            match env.as_str() {
                "always" => return Ok(Self::Always),
                "never" => return Ok(Self::Never),
                &_ => return Ok(Self::Auto),
            };
        } else if matches.get(&flags::QUOTES)?.is_some() {
        // prioritize the command line flag over the environment variable 
            let word = matches.get(&flags::QUOTES)?.unwrap();
            match word.to_str() {
                Some("always") => return Ok(Self::Always),
                Some("never") => return Ok(Self::Never),
                Some("auto") => return Ok(Self::Auto),
                None => return Err(OptionsError::BadArgument(&flags::QUOTES, word.into())),
                _ => return Err(OptionsError::BadArgument(&flags::QUOTES, word.into())),
            }
        } else {
            return Ok(Self::Auto);
        };
    }
}


impl EmbedHyperlinks {
    fn deduce(matches: &MatchedFlags<'_>) -> Result<Self, OptionsError> {
        let flagged = matches.has(&flags::HYPERLINK)?;

        if flagged {
            Ok(Self::On)
        } else {
            Ok(Self::Off)
        }
    }
}
