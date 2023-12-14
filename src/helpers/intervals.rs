use num::PrimInt;

pub struct Intervals<T: PrimInt> {
    pub intervals: Vec<(T, T)>,
}

impl<T: PrimInt> Intervals<T> {
    pub fn new() -> Self {
        Self {
            intervals: Vec::new(),
        }
    }

    pub fn add_interval(&mut self, start: T, end: T) {
        let mut new_intervals = Vec::new();
        let mut start = start;
        let mut end = end;
        for (i_start, i_end) in self.intervals.iter() {
            if start > *i_end {
                new_intervals.push((*i_start, *i_end));
            } else if end < *i_start {
                new_intervals.push((start, end));
                start = *i_start;
                end = *i_end;
            } else {
                start = start.min(*i_start);
                end = end.max(*i_end);
            }
        }
        new_intervals.push((start, end));
        self.intervals = new_intervals;
    }

    pub fn is_covered(&self, value: T) -> bool {
        for (start, end) in &self.intervals {
            if start > &value {
                return false;
            }
            if end >= &value {
                return true;
            }
        }
        false
    }

    pub fn get_covering_interval(&self, value: T) -> Option<(T, T)> {
        for (start, end) in &self.intervals {
            if start > &value {
                return None;
            }
            if end >= &value {
                return Some((*start, *end));
            }
        }
        None
    }
}
