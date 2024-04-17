use crate::{
    containers::{ParsePrompt, ParseStatus},
    string,
};

use super::ParseObjectError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Machine<'json> {
    In {
        postcomma: bool,
    },
    Name(Option<string::ParsedString<'json>>),
    PreElement {
        name: string::ParsedString<'json>,
    },
    Element {
        name: string::ParsedString<'json>,
        element: ParseStatus,
    },
    End,
}

impl<'json> Machine<'json> {
    pub fn apply(self, c: char) -> Result<Self, ParseObjectError> {
        match self {
            Self::In { postcomma } => match c {
                w if w.is_whitespace() => Ok(self),

                '"' => Ok(Self::Name(None)),

                '}' => {
                    if postcomma {
                        Err(ParseObjectError::TrailingComma)
                    } else {
                        Ok(Self::End)
                    }
                }

                _ => Err(ParseObjectError::ExpectedName {
                    c,
                    or_end: !postcomma,
                }),
            },

            Self::Name(Some(name)) => match c {
                w if w.is_whitespace() => Ok(self),

                ':' => Ok(Self::PreElement { name }),

                _ => Err(ParseObjectError::ExpectedColon(c)),
            },

            Self::PreElement { name } => {
                if c.is_whitespace() {
                    Ok(self)
                } else if let Some(prompt) = ParsePrompt::get(c) {
                    Ok(Self::Element {
                        name,
                        element: prompt.into(),
                    })
                } else {
                    Err(ParseObjectError::InvalidElement(c))
                }
            }

            Self::Element {
                name: _,
                element: ParseStatus::Done,
            } => match c {
                w if w.is_whitespace() => Ok(self),

                ',' => Ok(Self::In { postcomma: true }),
                '}' => Ok(Self::End),

                _ => Err(ParseObjectError::ExpectedCommaOrEnd(c)),
            },

            _ => panic!(),
        }
    }
}
