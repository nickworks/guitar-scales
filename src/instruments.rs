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
    pub fn from(name: &str, strings: Vec<usize>) -> Tuning {
        Tuning {
            name: name.to_string(),
            strings,
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
            tunings: vec![],
        }
    }
}
impl Instrument {
    pub fn from(name: &str, tunings: Vec<Tuning>) -> Instrument {
        Instrument {
            name: name.to_string(),
            tune_index: 0,
            tunings,
        }
    }
    pub fn strings(&self) -> &Vec<usize> {
        &self.tuning().strings
    }
    pub fn tuning(&self) -> &Tuning {
        &self.tunings[self.tune_index]
    }
    pub fn none() -> Instrument {
        Instrument::from("none", vec![])
    }
    pub fn guitar() -> Instrument {
        let standard = vec![4, 9, 14, 19, 23, 28];
        let shape_none = vec![0, 0, 0, 0, 0, 0];
        let shape_e = vec![0, 2, 2, 1, 0, 0];
        let shape_a = vec![0, 0, 2, 2, 2, 0];
        let shape_drop_d = vec![-2, 0, 0, 0, 0, 0];
        let shape_d = vec![-1, 0, 0, 2, 3, 2];
        Instrument::from("Guitar", vec![
                Tuning::from("Standard", offset_strings(&standard, &shape_none, 0, false)),
                Tuning::from("Standard (lefty)", offset_strings(&standard, &shape_none, 0, true)),
                Tuning::from("Standard Eb", offset_strings(&standard, &shape_none, -1, false)),
                Tuning::from("Standard D", offset_strings(&standard, &shape_none, -2, false)),
                Tuning::from("DROP D", offset_strings(&standard, &shape_drop_d, 0, false)),
                Tuning::from("OPEN D", offset_strings(&standard, &shape_e, -2, false)),
                Tuning::from("OPEN E", offset_strings(&standard, &shape_e, 0, false)),
                Tuning::from("OPEN F", offset_strings(&standard, &shape_a, -4, false)),
                Tuning::from("OPEN G", offset_strings(&standard, &shape_a, -2, false)),
                Tuning::from("OPEN A", offset_strings(&standard, &shape_a, 0, false)),
                Tuning::from("OPEN C", offset_strings(&standard, &shape_d, -3, false)),
            ],
        )
    }
    pub fn violin() -> Instrument {
        Instrument::from("Violin", vec![
            Tuning::from("Standard", vec![7, 14, 21, 28]),
        ])
    }
    pub fn cello() -> Instrument {
        Instrument::from("Cello", vec![
            Tuning::from("Standard", vec![0, 7, 14, 21]),
        ])
    }
    pub fn ukulele() -> Instrument {
        Instrument::from("Ukulele", vec![
            Tuning::from("Standard", vec![7, 12, 16, 21]),
        ])
    }
    pub fn banjo() -> Instrument {
        Instrument::from("Banjo", vec![
            Tuning::from("Standard", vec![7, 14, 19, 23, 26]),
        ])
    }
}
fn offset_strings(vec1: &[usize], vec2: &[i32], more:i32, is_left:bool) -> Vec<usize> {
    let mut offset = more;
    for i in 0..vec2.len() {
        while vec2[i] + offset < 0 {
            offset += 12;
        }
    }
    let mut result = Vec::with_capacity(vec1.len());
    for i in 0..vec1.len() {
        let plus = vec2[i] + offset;
        // println!("{:} + {:} - {:} = {:}", more, vec2[i], offset, plus);
        result.push(vec1[i] + plus as usize);
    }
    if is_left {
        result.reverse();
    }
    return result;
}
