use serde::{Deserialize, Serialize};

/// Comparison operations
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
pub enum CompareFunction {
    /// A == B
    Equal,
    /// A != B
    NotEqual,
    /// A < B
    Less,
    /// A <= B
    LessEqual,
    /// A > B
    Greater,
    /// A >= B
    GreaterEqual,
}

impl CompareFunction {
    /// Evaluate the operation with the given operands, `lhs` and `rhs`.
    /// Return the result as a `bool`.
    pub fn eval<T>(&self, lhs: &T, rhs: &T) -> bool
    where
        T: ?Sized,
        T: PartialOrd<T> + PartialEq<T>,
    {
        match self {
            CompareFunction::Equal => lhs == rhs,
            CompareFunction::NotEqual => lhs != rhs,
            CompareFunction::Less => lhs < rhs,
            CompareFunction::LessEqual => lhs <= rhs,
            CompareFunction::Greater => lhs > rhs,
            CompareFunction::GreaterEqual => lhs >= rhs,
        }
    }
}

/// [`CompareFunction`] with a partial parameter.
#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct PartialCompareFunction<T>(pub CompareFunction, pub T)
where
    T: ?Sized,
    T: PartialOrd<T> + PartialEq<T>;

impl<T> PartialCompareFunction<T>
where
    T: ?Sized,
    T: PartialOrd<T> + PartialEq<T>,
{
    pub fn eval_lhs(&self, lhs: &T) -> bool {
        self.0.eval(lhs, &self.1)
    }
    pub fn eval_rhs(&self, rhs: &T) -> bool {
        self.0.eval(&self.1, rhs)
    }
}
