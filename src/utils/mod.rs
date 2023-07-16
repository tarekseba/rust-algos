use std::{cmp::Ordering, fs, str::FromStr};

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

pub fn read_input<T>() -> Result<Vec<T>, String>
where
    T: FromStr,
{
    let data = fs::read_to_string("src/input.txt").unwrap();
    data.lines()
        .map(|line| {
            line.trim().parse::<T>()
                .map_err(|_| String::from("parsing failed"))
        })
        .collect()
}
