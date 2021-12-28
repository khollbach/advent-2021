use SymbolType::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Symbol {
    Open(Open),
    Close(Close),
}

/// One of ( [ { <
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Open(SymbolType);

/// One of ) ] } >
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Close(SymbolType);

#[derive(Copy, Clone, Eq, PartialEq)]
enum SymbolType {
    Round,
    Square,
    Curly,
    Angle,
}

impl Symbol {
    pub fn new(c: char) -> Self {
        let type_ = if "()".contains(c) {
            Round
        } else if "[]".contains(c) {
            Square
        } else if "{}".contains(c) {
            Curly
        } else {
            assert!("<>".contains(c), "Invalid symbol: {}", c);
            Angle
        };

        if "([{<".contains(c) {
            Symbol::Open(Open(type_))
        } else {
            debug_assert!(")]}>".contains(c));
            Symbol::Close(Close(type_))
        }
    }
}

impl Open {
    pub fn to_close(self) -> Close {
        let Open(type_) = self;
        Close(type_)
    }
}

impl Close {
    pub fn corruption_score(self) -> u32 {
        let Close(type_) = self;
        match type_ {
            Round => 3,
            Square => 57,
            Curly => 1197,
            Angle => 25137,
        }
    }

    pub fn completion_score(self) -> u64 {
        let Close(type_) = self;
        match type_ {
            Round => 1,
            Square => 2,
            Curly => 3,
            Angle => 4,
        }
    }
}
