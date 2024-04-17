use super::{ParseLiteralError, ParsedLiteral};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Machine {
    Start,
    End(ParsedLiteral),

    T,
    Tr,
    Tru,

    F,
    Fa,
    Fal,
    Fals,

    N,
    Nu,
    Nul,
}

impl Machine {
    pub fn apply(self, c: char) -> Result<Self, ParseLiteralError> {
        match self {
            Self::Start => match c {
                w if w.is_whitespace() => Ok(Self::Start),

                't' => Ok(Self::T),
                'f' => Ok(Self::F),
                'n' => Ok(Self::N),

                _ => Err(ParseLiteralError::UnexpectedCharacter(c)),
            },

            Self::T if c == 'r' => Ok(Self::Tr),
            Self::Tr if c == 'u' => Ok(Self::Tru),
            Self::Tru if c == 'e' => Ok(Self::End(ParsedLiteral::True)),

            Self::F if c == 'a' => Ok(Self::Fa),
            Self::Fa if c == 'l' => Ok(Self::Fal),
            Self::Fal if c == 's' => Ok(Self::Fals),
            Self::Fals if c == 'e' => Ok(Self::End(ParsedLiteral::False)),

            Self::N if c == 'u' => Ok(Self::Nu),
            Self::Nu if c == 'l' => Ok(Self::Nul),
            Self::Nul if c == 'l' => Ok(Self::End(ParsedLiteral::Null)),

            _ => Err(ParseLiteralError::UnexpectedCharacter(c)),
        }
    }
}
