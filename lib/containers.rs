use crate::{
    any::Any, array::Array, literal::Literal, number::Number, object::Object, string::String,
    Parent,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParsePrompt {
    String,
    Number,
    Object,
    Array,
    Literal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseStatus {
    Prompted(ParsePrompt),
    Done,
}

impl ParsePrompt {
    #[inline]
    pub const fn get(c: char) -> Option<Self> {
        match c {
            '"' => Some(Self::String),
            '0'..='9' | '-' => Some(Self::Number),
            '{' => Some(Self::Object),
            '[' => Some(Self::Array),
            't' | 'f' | 'n' => Some(Self::Literal),

            _ => None,
        }
    }

    #[inline]
    pub const fn keep_first(self) -> bool {
        matches!(self, Self::Number | Self::Literal)
    }

    pub fn create<'json, 'p>(
        self,
        parent: &'p mut dyn Parent<'json>,
        remaining: &'json str,
    ) -> Any<'json, 'p>
    where
        'json: 'p,
    {
        match self {
            Self::String => Any::String(String::new(parent, remaining)),
            Self::Number => Any::Number(Number::new(parent, remaining)),
            Self::Object => Any::Object(Object::new(parent, remaining)),
            Self::Array => Any::Array(Array::new(parent, remaining)),
            Self::Literal => Any::Literal(Literal::new(parent, remaining)),
        }
    }
}

impl From<ParsePrompt> for ParseStatus {
    #[inline]
    fn from(value: ParsePrompt) -> Self {
        Self::Prompted(value)
    }
}
