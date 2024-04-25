#![macro_use]

pub struct ScaleIntervals {
    pub name: String,
    pub isMinor: bool,
    pub notes: Vec<usize>,
}
macro_rules! music_scale {
    ($name:literal,$minor:literal,$($note:literal),+) => {
        ScaleIntervals {
            name: $name.to_string(),
            isMinor: $minor,
            notes: vec![$($note),+],
        }
    };
}