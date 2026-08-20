use std::fmt::{Display, Formatter, Debug};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

type IndexType = u64;
type AtomicType = AtomicU64; // Should match

static CURRENT_INDEX: AtomicType = AtomicType::new(0);

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct NoiseSymbol(IndexType);

impl NoiseSymbol {
    pub fn new() -> Self {
        Self(CURRENT_INDEX.fetch_add(1, Relaxed))
    }
}

impl Display for NoiseSymbol {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "ε{}", self.0)
    }
}

impl Debug for NoiseSymbol {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "ε{}", self.0)
    }
}
