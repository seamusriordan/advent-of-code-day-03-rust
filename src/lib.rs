mod tests;


pub fn vec_to_usize(v: &Vec<usize>) -> usize {
    let mut x = 0;
    for i in 0..(v.len()) {
        x += v[v.len() - i - 1] << i;
    }
    x
}

pub fn number_string_to_bits(number: &str) -> Vec<usize> {
    let mut v = vec![];

    for b in number.chars().into_iter() {
        match b {
            '0' => v.push(0),
            '1' => v.push(1),
            _ => panic!("Bad number")
        }
    }
    v
}


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
        for _ in 0..size {
            proto.counts.push(0)
        }
        proto
    }

    pub fn add_bits(&mut self, bit_string: &str) {
        let mut index = 0;
        for b in number_string_to_bits(bit_string) {
            match b {
                1 => self.counts[index] += 1,
                _ => {}
            }
            index += 1;
        }
        self.n_additions += 1
    }

    fn get_by_discriminator(&self, discriminator: impl Fn(usize) -> bool) -> Vec<usize> {
        let mut output = vec![];

        for i in 0..self.size {
            if discriminator(self.counts[i]) {
                output.push(1)
            } else {
                output.push(0)
            }
        }
        return output;
    }

    pub fn get_most_common(&self) -> Vec<usize> {
        self.get_by_discriminator(|x| {x >= self.n_additions - x})
    }

    pub fn get_least_common(&self) -> Vec<usize> {
       self.get_by_discriminator(|x| {x < self.n_additions - x })
    }
}


fn get_number_for_reference_method<'a>(
    bits: impl Iterator<Item = &'a str>,
    f: impl Fn(&BitCounter) -> Vec<usize>
) -> usize {
    let mut numbers: Vec<&str> = bits.collect();

    let mut index: usize = 0;
    while numbers.len() > 1 {
        let mut bit_counter = BitCounter::new(12);
        for number in numbers.iter() {
            bit_counter.add_bits(number);
        }
        let reference_number = f(&bit_counter);
        numbers = numbers.into_iter().filter(|number|
            reference_number[index] == number_string_to_bits(number)[index]
        ).collect();
        index += 1;
    }
    vec_to_usize(&number_string_to_bits(numbers[0]))
}

pub fn get_oxygen<'a>(bits: impl Iterator<Item = &'a str>) -> usize {
    get_number_for_reference_method(bits, BitCounter::get_most_common)
}

pub fn get_co2<'a>(bits: impl Iterator<Item = &'a str>) -> usize {
    get_number_for_reference_method(bits, BitCounter::get_least_common)
}


