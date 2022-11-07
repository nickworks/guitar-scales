#![macro_use]

pub struct ScaleIntervals {
    pub name: String,
    pub notes: Vec<usize>,
}
macro_rules! music_scale {
    ($name:literal, $($note:literal),+) => {
        ScaleIntervals {
            name: $name.to_string(),
            notes: vec![$($note),+],
        }
    };
}