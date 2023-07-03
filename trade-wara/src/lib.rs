mod entities;

use std::{cmp::Ordering, ops::Deref};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ComparableFloat(pub f32);

impl Eq for ComparableFloat {}

impl Ord for ComparableFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            return Ordering::Less;
        }

        if self > other {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

impl From<f32> for ComparableFloat {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Deref for ComparableFloat {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
