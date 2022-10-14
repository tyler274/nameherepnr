use bitvec::vec::BitVec;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
#[repr(u8)]
pub enum State {
    // Defined values are one of the two below.
    S0 = b'0',
    S1 = b'1',
    // These are values that may show up in undefined.
    Sx = b'x',
    Sz = b'z',
}

impl TryFrom<char> for State {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(State::S0),
            '1' => Ok(State::S1),
            'x' => Ok(State::Sx),
            'z' => Ok(State::Sz),
            _ => Err("State only accepts values in the character set {0,1,x,z}"),
        }
    }
}

impl From<State> for char {
    fn from(value: State) -> Self {
        value.to_char()
    }
}

impl TryFrom<u8> for State {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'0' => Ok(State::S0),
            b'1' => Ok(State::S1),
            b'x' => Ok(State::Sx),
            b'z' => Ok(State::Sz),
            _ => Err("State only accepts values in the character set {0,1,x,z}"),
        }
    }
}

impl TryFrom<Option<char>> for State {
    type Error = &'static str;

    fn try_from(value: Option<char>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => match v {
                '0' => Ok(State::S0),
                '1' => Ok(State::S1),
                'x' => Ok(State::Sx),
                'z' => Ok(State::Sz),
                _ => Err("State only accepts values in the character set {0,1,x,z}"),
            },
            None => Err("State can't be converted from a None"),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl State {
    pub const fn to_char(&self) -> char {
        match self {
            State::S0 => '0',
            State::S1 => '1',
            State::Sx => 'x',
            State::Sz => 'z',
        }
    }
}

//impl Display for State<i64> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(
//            f,
//            "{}",
//            match self {
//                State::S0(_) => '0',
//                State::S1(_) => '1',
//                State::Sx(_) => 'x',
//                State::Sz(_) => 'z',
//            }
//        )
//    }
//}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Property {
    Str(State, String),
    Int(State, i64, String),
}
// pub struct Property {
//     is_string: bool,
//     // The string literal (for string values), or a string of [01xz] (for numeric values)
//     str: String,
//     // The lower 64 bits (for numeric values), unused for string values
//     intval: i64,
// }

// int64_t as_int64() const
impl TryFrom<Property> for i64 {
    type Error = &'static str;

    fn try_from(value: Property) -> Result<Self, Self::Error> {
        match value {
            Property::Str(_state, _strval) => Err("Cannot convert a String Property to an int64"),
            Property::Int(_state, intval, _strval) => Ok(intval),
        }
    }
}

//impl TryFrom<Property> for Vec<bool> {
//    type Error = &'static str;
//
//    fn try_from(value: Property) -> Result<Self, Self::Error> {
//        match value {
//            Property::Int(_, intval , strval) => {
//                Ok({
//
//                })
//            },
//            Property::Str(_, _) => {
//                Err("Cannot convert Strong Property to a")
//            }
//        }
//    }
//}

impl TryFrom<Property> for BitVec {
    type Error = &'static str;

    fn try_from(value: Property) -> Result<Self, Self::Error> {
        match value {
            Property::Int(_, intval, strval) => Ok({
                let mut result = BitVec::with_capacity(strval.len());
                for c in strval.chars() {
                    result.push(c == State::S1.into());
                }
                result
            }),
            Property::Str(_, _) => Err("Cannot convert Strong Property to a FixedBitSet"),
        }
    }
}

impl TryFrom<Property> for String {
    type Error = &'static str;
    fn try_from(value: Property) -> Result<Self, Self::Error> {
        match value {
            Property::Int(_, _, _) => Err("Cannot convert i64 Property into String"),
            Property::Str(_, strval) => Ok(strval),
        }
    }
}

impl From<Property> for bool {
    fn from(value: Property) -> Self {
        match value {
            Property::Int(_, intval, strval) => intval != 0,
            Property::Str(_, strval) => strval.contains(State::S1.to_char()),
        }
    }
}

impl Property {
    pub const fn default() -> Self {
        // Self {
        //     is_string: false,
        //     Str: String::new(),
        //     intval: 0,
        // }
        Self::Int(State::S0, 0, String::new())
    }

    pub fn with_width(intval: i64, width: usize) -> Self {
        Self::Int(State::S0, intval, {
            let mut strval = String::with_capacity(width);
            for i in 0..width {
                strval.push(if (intval & (1 << i) != 0) {
                    State::S1.into()
                } else {
                    State::S0.into()
                })
            }
            strval
        })
    }

    pub fn with_str(strval: &str) -> Self {
        Property::Str(State::S0, strval.to_string())
    }

    pub fn with_state(bit: State) -> Self {
        Property::Int(
            bit,
            match bit {
                State::S1 => 1,
                _ => 0,
            },
            { String::from(bit.to_char()) },
        )
    }

    /// Convert to a string representation, escaping literal strings matching /^[01xz]* *$/ by adding a space at the end,
    /// to disambiguate from binary strings
    pub fn to_string(&self) -> String {
        match self {
            Self::Str(state, strval) => {
                let mut result = strval.clone();
                let mut state = 0;
                for c in strval.chars() {
                    if state == 0 {
                        if (c == '0' || c == '1' || c == 'x' || c == 'z') {
                            state = 0;
                        } else if c == ' ' {
                            state = 1;
                        } else {
                            state = 2;
                        }
                    } else if (state == 1 && c != ' ') {
                        state = 2;
                    }
                }
                if state < 2 {
                    result.push(' ');
                }
                result
            }
            Self::Int(_state, _intval, strval) => strval.chars().rev().collect(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Property::Str(_, strval) => 8 * strval.len(),
            Property::Int(_, _, strval) => strval.len(),
        }
    }
    pub fn update_intval(&mut self) {
        match self {
            Self::Str(state, strval) => {
                todo!()
            }
            Self::Int(state, intval, strval) => {
                *intval = 0;
                for i in 0..strval.len() {
                    // TODO: This is not a good way to do this conversion, implement a conversion to character.
                    assert!(
                        strval.chars().nth(i).unwrap() == State::S0.into()
                            || strval.chars().nth(i).unwrap() == State::S1.into()
                            || strval.chars().nth(i).unwrap() == State::Sx.into()
                            || strval.chars().nth(i).unwrap() == State::Sz.into()
                    );
                    if strval.chars().nth(i).unwrap() == State::S1.into() && i < 64 {
                        *intval |= 1 << i;
                    }
                }
            }
        }
    }

    pub fn is_fully_def(&self) -> bool {
        match self {
            Self::Str(state, strval) => {
                todo!()
            }
            Self::Int(state, intval, strval) => {
                match strval
                    .chars()
                    .find(|&x| x == State::Sx.into() || x == State::Sz.into())
                {
                    Some(_) => false,
                    None => true,
                }
            }
        }
    }
    pub fn extract(&self, offset: usize, len: usize, padding: State) -> Self {
        let mut ret = Property::default();
        match &mut ret {
            Property::Int(state, intval, strval) => {
                *strval = String::with_capacity(len);
                for i in offset..offset + len {
                    strval.push(if i < strval.len() {
                        strval.chars().nth(i).unwrap()
                    } else {
                        padding.to_char()
                    });
                }
            }
            _ => {
                todo!()
            }
        }
        ret.update_intval();

        ret
    }

    /// Convert a string of four-value binary [01xz], or a literal string escaped according to the above rule
    /// to a Property
    pub fn from_string(s: &str) -> Self {
        let mut p = Property::default();
        let cursor = s.find(|c: char| {
            c != State::S0.to_char()
                && c != State::S1.to_char()
                && c != State::Sx.to_char()
                && c != State::Sz.to_char()
        });
        todo!()
        match cursor {
            None => {},
        }

        p
    }
}
