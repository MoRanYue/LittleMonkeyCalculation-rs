use std::cmp::Ordering;

#[derive(Debug)]
pub struct Comparation(pub i32, pub i32);

impl Comparation {
    pub fn compare(&self) -> Ordering {
        self.0.cmp(&self.1)
    }
}

impl Default for Comparation {
    fn default() -> Self {
        Self(-1, -1)
    }
}

pub fn prepare_comparation_input(text: String) -> Result<Comparation, &'static str> {
    let mut comparation = Comparation::default();
    let texts = text.replace("?", " ");
    let mut nums = texts.split_ascii_whitespace().map(|num| num.parse::<i32>());
    if let Some(n) = nums.next() {
        if let Ok(n) = n {
            comparation.0 = n;
        }
        else {
            return Err("cannot create a comparation of non-numbers");
        }
    }
    else {
        return Err("number is not enough");
    }
    if let Some(n) = nums.next() {
        if let Ok(n) = n {
            comparation.1 = n;
        }
        else {
            return Err("cannot create a comparation of non-numbers");
        }
    }
    else {
        return Err("number is not enough");
    }
    Ok(comparation)
}