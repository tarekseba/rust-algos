use std::cmp::Ordering;

pub mod thread_pools;

#[derive(Clone)]
pub enum Sorting {
    Ascending,
    Descending,
}

impl Sorting {
    pub fn flip(&self) -> Sorting {
        match &self {
            Sorting::Ascending => Sorting::Descending,
            Sorting::Descending => Sorting::Ascending,
        }
    }
}

impl From<&Sorting> for Ordering {
    fn from(value: &Sorting) -> Self {
        match value {
            Sorting::Descending => Self::Less,
            _ => Self::Greater,
        }
    }
}
