pub struct Instrument {
    pub name: String,
    pub tune_index: usize,
    pub tunings: Vec<Tuning>,
}
pub struct Tuning {
    pub name: String,
    pub strings: Vec<usize>,
}
impl Tuning {
    pub fn from(name: &str, mut strings: Vec<usize>, offset: Vec<usize>) -> Tuning {
        for i in 0..strings.len() {
            if i >= offset.len() {
                break;
            }
            strings[i] += offset[i];
        }
        Tuning {
            name: name.to_string(),
            strings: strings,
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}
impl Default for Instrument {
    fn default() -> Self {
        Self {
            name: "None".to_string(),
            tune_index: 0,
            tunings: vec!(),
        }
    }
}
impl Instrument {
    pub fn from(name: &str, tunings: Vec<Tuning>) -> Instrument {
        Instrument {
            name: name.to_string(),
            tune_index: 0,
            tunings: tunings,
        }
    }
    pub fn strings(&self) -> &Vec<usize> {
        &self.tuning().strings
    }
    pub fn tuning(&self) -> &Tuning {
        &self.tunings[self.tune_index]
    }
    pub fn none() -> Instrument {
        Instrument::from("none", vec!())
    }
    pub fn guitar() -> Instrument {
        let standard = vec!(4,9,14,19,23,28);
        Instrument::from("Guitar", vec!(
            Tuning::from("Standard", standard.to_vec(), vec!()),
            Tuning::from("Standard (lefty)", vec!(28, 23, 19, 14, 9, 4), vec!()),
            Tuning::from("Standard D", standard.to_vec(), vec!(10, 10, 10, 10, 10, 10)),
            Tuning::from("DROP D", standard.to_vec(), vec!(10, 12, 12, 12, 12, 12)),
            Tuning::from("OPEN D", standard.to_vec(), vec!(10, 12, 12, 11, 10, 10)),
            Tuning::from("OPEN E", standard.to_vec(), vec!( 0,  2,  2,  1,  0,  0)),
            Tuning::from("OPEN F", standard.to_vec(), vec!( 8,  8, 10, 10, 10,  8)),
            Tuning::from("OPEN G", standard.to_vec(), vec!(10, 10, 12, 12, 12, 10)),
            Tuning::from("OPEN C", standard.to_vec(), vec!( 8, 10, 10, 12, 13, 12)),
        ))
    }
    pub fn violin() -> Instrument {
        Instrument::from("Violin", vec!(
            Tuning::from("Standard", vec![7,14,21,28], vec!()),
        ))
    }
    pub fn cello() -> Instrument {
        Instrument::from("Cello", vec!(
            Tuning::from("Standard", vec![0,7,14,21], vec!()),
        ))
    }
    pub fn ukulele() -> Instrument {
        Instrument::from("Ukulele", vec!(
            Tuning::from("Standard", vec![7,12,16,21], vec!()),
        ))
    }
    pub fn banjo() -> Instrument {
        Instrument::from("Banjo", vec!(
            Tuning::from("Standard", vec![7,14,19,23,26], vec!()),
        ))
    }
}