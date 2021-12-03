#[cfg(test)]
mod tests {
    use crate::{BitCounter, vec_to_usize};

    #[test]
    fn new_has_0_counts() {
        let counter = BitCounter::new(5);

        assert_eq!(counter.counts, vec![0, 0, 0, 0, 0]);
    }


    #[test]
    fn input_00001_counts1() {
        let mut counter = BitCounter::new(5);

        counter.add_bits("00001");

        assert_eq!(counter.counts, vec![0, 0, 0, 0, 1]);
    }

    #[test]
    fn input_00001_has_most_common_index_4() {
        let mut counter = BitCounter::new(5);

        counter.add_bits("00001");

        assert_eq!(vec![0, 0, 0, 0, 1], counter.get_most_common());
    }

    #[test]
    fn input_00001_10000_has_no_most_common() {
        let mut counter = BitCounter::new(5);

        counter.add_bits("00001");
        counter.add_bits("10000");

        assert_eq!(vec![0, 0, 0, 0, 0], counter.get_most_common());
    }

    #[test]
    fn input_00001_10000_10001_has_most_common_0_4() {
        let mut counter = BitCounter::new(5);

        counter.add_bits("00001");
        counter.add_bits("10000");
        counter.add_bits("10001");

        assert_eq!(vec![1, 0, 0, 0, 1], counter.get_most_common());
    }

    #[test]
    fn input_00001_10000_10001_has_least_common_1_2_3() {
        let mut counter = BitCounter::new(5);

        counter.add_bits("00001");
        counter.add_bits("10000");
        counter.add_bits("10001");

        assert_eq!(vec![0, 1, 1, 1, 0], counter.get_least_common());
    }

    #[test]
    fn string_00000_is_0() {
        assert_eq!(0, vec_to_usize(vec![0, 0, 0, 0, 0]));
    }

    #[test]
    fn string_10000_is_16() {
        assert_eq!(16, vec_to_usize(vec![1, 0, 0, 0, 0]));
    }

    #[test]
    fn string_10110_is_22() {
        assert_eq!(22, vec_to_usize(vec![1, 0, 1, 1, 0]));
    }

    #[test]
    fn example_gives_198() {
        let mut counter = BitCounter::new(5);

        let input_strings = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010 ",
        ];


        for input in input_strings {
            counter.add_bits(input);
        }

        assert_eq!(22, vec_to_usize(counter.get_most_common()));
        assert_eq!(9, vec_to_usize(counter.get_least_common()));
    }
}
