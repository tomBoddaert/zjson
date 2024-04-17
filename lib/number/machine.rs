use crate::status::Status;

use super::ParseNumberError;

pub enum Machine {
    Start { signed: bool },
    InInteger,
    PostInteger,
    PreFraction,
    Fraction,
    PreExponent { signed: bool },
    Exponent,
}

impl Machine {
    pub const fn apply(self, c: char) -> Result<Status<Self, ()>, ParseNumberError> {
        match self {
            Self::Start { signed } => Ok(Status::Parsing(match c {
                '-' if !signed => Self::Start { signed: true },
                '1'..='9' => Self::InInteger,
                '0' => Self::PostInteger,

                _ => return Err(ParseNumberError::ExpectedMinusOrDigit(c)),
            })),

            Self::InInteger => Ok(Status::Parsing(match c {
                '0'..='9' => Self::InInteger,
                '.' => Self::PreFraction,
                'e' | 'E' => Self::PreExponent { signed: false },

                _ => return Ok(Status::Done(())),
            })),

            Self::PostInteger => Ok(Status::Parsing(match c {
                '.' => Self::PreFraction,
                'e' | 'E' => Self::PreExponent { signed: false },

                _ => return Ok(Status::Done(())),
            })),

            Self::PreFraction => {
                if c.is_ascii_digit() {
                    Ok(Status::Parsing(Self::Fraction))
                } else {
                    Err(ParseNumberError::ExpectedDigit(c))
                }
            }

            Self::Fraction => Ok(Status::Parsing(match c {
                '0'..='9' => Self::Fraction,
                'e' | 'E' => Self::PreExponent { signed: false },

                _ => return Ok(Status::Done(())),
            })),

            Self::PreExponent { signed } => Ok(Status::Parsing(match c {
                '-' | '+' if !signed => Self::PreExponent { signed: true },
                '0'..='9' => Self::Exponent,

                _ => return Err(ParseNumberError::ExpectedSignOrDigit(c)),
            })),

            Self::Exponent => Ok(if c.is_ascii_digit() {
                Status::Parsing(Self::Exponent)
            } else {
                Status::Done(())
            }),
        }
    }

    pub const fn valid_end(self) -> Result<(), ParseNumberError> {
        match self {
            Self::InInteger | Self::PostInteger | Self::Fraction | Self::Exponent => Ok(()),

            Self::Start { signed } => Err(ParseNumberError::UnexpectedEnd { or_sign: !signed }),
            Self::PreFraction => Err(ParseNumberError::UnexpectedEnd { or_sign: false }),
            Self::PreExponent { signed } => {
                Err(ParseNumberError::UnexpectedEndAfterExponent { or_sign: !signed })
            }
        }
    }
}
