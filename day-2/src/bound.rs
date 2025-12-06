#[derive(Debug)]
pub struct Range {
    pub min: Bound,
    pub max: Bound,
}

impl Range {
    pub fn new(min: Bound, max: Bound) -> Self {
        Self { min, max }
    }

    pub fn has_equal_digits(&self) -> bool {
        self.min.digits == self.max.digits
    }
}

impl PartialEq<Range> for Range {
    fn eq(&self, other: &Range) -> bool {
        return self.min.value == other.min.value && self.max.value == other.max.value;
    }
}

#[derive(Debug, Clone)]
pub struct Bound {
    pub value: usize,
    pub digits: usize,
    pub str: Box<str>,
}

impl Bound {
    pub fn has_even_digits(&self) -> bool {
        return self.digits % 2 == 0;
    }
}

impl From<String> for Bound {
    fn from(number: String) -> Self {
        Self {
            value: number.parse::<usize>().expect("Valid number"),
            digits: number.chars().count(),
            str: number.into(),
        }
    }
}

impl From<usize> for Bound {
    fn from(number: usize) -> Self {
        let string: Box<str> = number.to_string().into();

        Self {
            value: number,
            digits: string.chars().count(),
            str: string,
        }
    }
}

impl From<&str> for Bound {
    fn from(number: &str) -> Self {
        Self {
            value: number.parse::<usize>().expect("Valid number"),
            digits: number.chars().count(),
            str: Box::from(number),
        }
    }
}
