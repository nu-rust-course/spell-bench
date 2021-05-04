#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Correction<S> {
    Correct,
    Uncorrectable,
    Suggestion(S)
}

macro_rules! matches {
    ($e:expr, $p:pat) => { if let $p = $e {true} else {false} };
}

impl<S> Correction<S> {
    pub fn from_result(result: Result<S, bool>) -> Self {
        result.map_or_else(Self::no_suggestion, Self::Suggestion)
    }

    fn no_suggestion(is_good: bool) -> Self {
        if is_good {Self::Correct} else {Self::Uncorrectable}
    }

    pub fn is_correct(&self) -> bool {
        matches!(self, Correction::Correct)
    }

    pub fn is_uncorrectable(&self) -> bool {
        matches!(self, Correction::Uncorrectable)
    }

    pub fn is_suggestion(&self) -> bool {
        matches!(self, Correction::Suggestion(_))
    }

    pub fn as_result(&self) -> Result<&S, bool> {
        match self {
            Self::Correct => Err(true),
            Self::Uncorrectable => Err(false),
            Self::Suggestion(word) => Ok(&word),
        }
    }

    pub fn as_option(&self) -> Option<&S> {
        self.as_result().ok()
    }

    pub fn into_result(self) -> Result<S, bool> {
        match self {
            Self::Correct => Err(true),
            Self::Uncorrectable => Err(false),
            Self::Suggestion(word) => Ok(word),
        }
    }

    pub fn into_option(self) -> Option<S> {
        self.into_result().ok()
    }

    pub fn unwrap_or(self, correct: S, uncorrectable: S) -> S {
        self.unwrap_or_else(|is_good| if is_good {correct} else {uncorrectable})
    }

    pub fn unwrap_or_else(self, or_else: impl FnOnce(bool) -> S) -> S {
        self.into_result().map_or_else(or_else, |s| s)
    }

    pub fn map_or_else<R>(self, or_else: impl FnOnce(bool) -> R, f: impl FnOnce(S) -> R) -> R {
        self.into_result().map_or_else(or_else, f)
    }

    pub fn map<T, F>(self, f: impl FnOnce(S) -> T) -> Correction<T> {
        Correction::from_result(self.into_result().map(f))
    }
}

