use crate::{status::Status, string::ParseStringError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Machine {
    In,
    Escape(EscapeMachine),
}

impl Machine {
    pub fn apply(self, c: char) -> Result<Option<Self>, ParseStringError> {
        match self {
            Self::In => Ok(match c {
                '\\' => Some(Self::Escape(EscapeMachine::Awaiting)),
                '"' => None,
                _ => Some(Self::In),
            }),

            Self::Escape(machine) => Ok(match machine.apply(c)? {
                Status::Parsing(machine) => Some(Self::Escape(machine)),
                Status::Done(_) => Some(Self::In),
            }),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EscapeMachine {
    Awaiting,
    Unicode { n: u16, len: u8 },
    Surrogate { high: u16, low: LowMachine },
}

impl EscapeMachine {
    pub fn apply(self, c: char) -> Result<Status<Self, char>, ParseStringError> {
        match self {
            Self::Awaiting => match c {
                '"' | '\\' | '/' => Ok(Status::Done(c)),
                'b' => Ok(Status::Done('\x08')),
                'f' => Ok(Status::Done('\x0c')),
                'n' => Ok(Status::Done('\n')),
                'r' => Ok(Status::Done('\r')),
                't' => Ok(Status::Done('\t')),

                'u' => Ok(Status::Parsing(Self::Unicode { n: 0, len: 0 })),

                _ => Err(ParseStringError::InvalidEscape(c)),
            },

            Self::Unicode { mut n, len } => {
                n <<= 4;
                // This won't truncate because the maximum result is 15
                #[allow(clippy::cast_possible_truncation)]
                {
                    n |= c
                        .to_digit(16)
                        .ok_or(ParseStringError::InvalidUnicodeEscape(c))?
                        as u16;
                }

                if len == 3 {
                    if let Some(char) = char::from_u32(u32::from(n)) {
                        return Ok(Status::Done(char));
                    }

                    // For u16s, the above only fails for surrogates
                    if n >= 0xdc00 {
                        return Err(ParseStringError::MissingHighSurrogate { low: n });
                    }

                    Ok(Status::Parsing(Self::Surrogate {
                        high: n,
                        low: LowMachine::Awaiting,
                    }))
                } else {
                    Ok(Status::Parsing(Self::Unicode { n, len: len + 1 }))
                }
            }

            Self::Surrogate { high, low } => low.apply(c, high).map(|status| match status {
                Status::Parsing(low) => Status::Parsing(Self::Surrogate { high, low }),
                Status::Done(parsed) => Status::Done(parsed),
            }),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowMachine {
    Awaiting,
    AwaitingU,
    Hex { low: u16, len: u8 },
}

impl LowMachine {
    fn apply(self, c: char, high: u16) -> Result<Status<Self, char>, ParseStringError> {
        match self {
            Self::Awaiting if c == '\\' => Ok(Status::Parsing(Self::AwaitingU)),
            Self::AwaitingU if c == 'u' => Ok(Status::Parsing(Self::Hex { low: 0, len: 0 })),

            Self::Hex { mut low, len } => {
                low <<= 4;
                // This won't truncate because the maximum result is 15
                #[allow(clippy::cast_possible_truncation)]
                {
                    low |= c
                        .to_digit(16)
                        .ok_or(ParseStringError::InvalidUnicodeEscape(c))?
                        as u16;
                }

                if len == 3 {
                    if !(0xdc00..0xe000).contains(&low) {
                        return Err(ParseStringError::InvalidLowSurrogate { high, low });
                    }

                    let char_code =
                        (u32::from(high - 0xd800) << 10) + u32::from(low - 0xdc00) + 0x10000;

                    // All surrogate pairs decode to valid char codes
                    let decoded =
                        char::from_u32(char_code).expect("failed to parse surrogate pair");

                    Ok(Status::Done(decoded))
                } else {
                    Ok(Status::Parsing(Self::Hex { low, len: len + 1 }))
                }
            }

            _ => Err(ParseStringError::MissingLowSurrogate { high }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{EscapeMachine, Machine};

    #[test]
    fn escaped_quotes() {
        let mut machine = Machine::In;

        machine = machine
            .apply('\\')
            .expect("failed to apply '\\' to machine")
            .expect("expected machine to continue");
        assert_eq!(machine, Machine::Escape(EscapeMachine::Awaiting));

        machine = machine
            .apply('"')
            .expect("failed to apply '\"' to machine")
            .expect("expected machine to continue");
        assert_eq!(machine, Machine::In);
    }

    #[test]
    fn string() {
        let mut machine = Machine::In;

        for c in "Hello, World!".chars() {
            machine = machine
                .apply(c)
                .expect("failed to apply character to machine")
                .expect("expected machine to continue");
        }

        let result = machine.apply('"').expect("failed to apply '\"' to machine");
        assert!(result.is_none());
    }

    #[test]
    fn escaped_string() {
        let mut machine = Machine::In;

        for c in r#"Hello\" World!"#.chars() {
            machine = machine
                .apply(c)
                .expect("failed to apply character to machine")
                .expect("expected machine to continue");
        }

        let result = machine.apply('"').expect("failed to apply '\"' to machine");
        assert!(result.is_none());
    }
}
