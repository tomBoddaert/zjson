use crate::containers::{ParsePrompt, ParseStatus};

use super::ParseArrayError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Machine {
    In { postcomma: bool },
    Element(ParseStatus),
    End,
}

impl Machine {
    pub fn apply(self, c: char) -> Result<Self, ParseArrayError> {
        match self {
            Self::In { postcomma } => match c {
                w if w.is_whitespace() => Ok(self),

                ']' => {
                    if postcomma {
                        Err(ParseArrayError::TrailingComma)
                    } else {
                        Ok(Self::End)
                    }
                }

                _ => ParsePrompt::get(c)
                    .map(|prompt| Self::Element(prompt.into()))
                    .ok_or(ParseArrayError::InvalidElement {
                        c,
                        or_end: !postcomma,
                    }),
            },

            Self::Element(ParseStatus::Done) => match c {
                w if w.is_whitespace() => Ok(self),

                ',' => Ok(Self::In { postcomma: true }),
                ']' => Ok(Self::End),

                _ => Err(ParseArrayError::ExpectedCommaOrEnd(c)),
            },

            _ => panic!(),
        }
    }
}
