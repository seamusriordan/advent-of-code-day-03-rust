mod tests;

pub struct BitCounter {
    pub counts: Vec<usize>,
    n_additions: usize,
    size: usize,
}

impl BitCounter {
    pub fn new(size: usize) -> BitCounter {
        let mut proto = BitCounter {
            counts: vec![],
            n_additions: 0,
            size
        };
        proto.reset();
        proto
    }

    fn reset(&mut self) {
        self.counts = vec![];
        for _ in 0..self.size {
            self.counts.push(0)
        }
        self.n_additions = 0
    }

    pub fn add_bits(&mut self, bit_string: &str) {
        let mut index = 0;
        for c in bit_string.chars() {
            match c {
                '1' => self.counts[index] += 1,
                _ => {}
            }
            index += 1;
        }
        self.n_additions += 1
    }

    pub fn get_most_common(&self) -> Vec<usize> {
        let mut output = vec![];

        for i in 0..self.size {
            if self.counts[i] > self.n_additions / 2 {
                output.push(1)
            } else {
                output.push(0)
            }
        }
        return output;
    }

    pub fn get_least_common(&self) -> Vec<usize> {
        let mut output = vec![];

        for i in 0..self.size {
            if self.counts[i] < self.n_additions / 2 {
                output.push(1)
            } else {
                output.push(0)
            }
        }
        return output;
    }
}

pub fn vec_to_usize(v: Vec<usize>) -> usize {
    let mut x = 0;
    for i in 0..(v.len()) {
        x += v[v.len() - i - 1] << i;
    }
    x
}