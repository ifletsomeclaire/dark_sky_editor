#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}
impl std::ops::Add for Range {
    type Output = Self;
    /// will become the full range represented across self and other
    fn add(self, other: Self) -> Self {
        Self {
            start: if self.start < other.start {
                self.start
            } else {
                other.start
            },
            end: if self.end > other.end {
                self.end
            } else {
                other.end
            },
        }
    }
}
impl Range {
    pub fn iter(&self) -> std::ops::Range<usize> {
        std::ops::Range {
            start: self.start,
            end: self.end,
        }
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

impl From<std::ops::Range<usize>> for Range {
    fn from(r: std::ops::Range<usize>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}
impl From<Range> for std::ops::Range<usize> {
    fn from(r: Range) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let s = Range { start: 10, end: 20 };
        let o = Range { start: 20, end: 26 };
        // let y: std::ops::Range<usize> = o.into();
        // let z: Range = y.into();

        let x = s + o;
        for i in o.iter() {}
        for i in o.iter() {}
    
        // println!("{:?} {:?} {:?} {:?} {:?}", x , y , z , s , o);
    }
}
