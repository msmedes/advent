fn main() -> anyhow::Result<()> {
    let input: Vec<Vec<usize>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.trim())
        .map(|line| separate_bits(line))
        .collect();

    // let input: Vec<Vec<usize>> =
    //     "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
    //         .lines()
    //         .map(|line| line.trim())
    //         .map(|line| separate_bits(line))
    //         .collect();

    let power = part1(&input);
    dbg!(power);

    let life_support_rating = part2(input);
    dbg!(life_support_rating);

    Ok(())
}

fn part1(bits: &[Vec<usize>]) -> usize {
    let (gamma, epsilon) = get_gamma_and_epsilon(bits);

    get_decimal_rate_from_binary(&gamma) * get_decimal_rate_from_binary(&epsilon)
}

fn part2(bits: Vec<Vec<usize>>) -> usize {
    let (oxygen, cO2) = get_oxygen_and_co2(bits);

    get_decimal_rate_from_binary(&oxygen) * get_decimal_rate_from_binary(&cO2)
}

fn get_oxygen_and_co2(bits: Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
    let mut oxygen: Option<Vec<usize>> = None;
    let mut cO2: Option<Vec<usize>> = None;

    let mut oxygen_bits = bits.clone();
    let mut cO2_bits = bits.clone();

    for i in 0..bits[0].len() {
        if oxygen == None {
            let (most_oxygen, _) =
                most_and_least_common_bit_for_index_with_tiebreaker(&oxygen_bits, i);
            let most_vals = get_numbers_with_value(oxygen_bits.clone(), most_oxygen, i);
            if most_vals.len() == 1 {
                oxygen = Some(most_vals[0].clone());
            } else {
                oxygen_bits = most_vals;
            }
        }
        if cO2 == None {
            let (_, least_cO2) = most_and_least_common_bit_for_index_with_tiebreaker(&cO2_bits, i);
            let least_vals = get_numbers_with_value(cO2_bits.clone(), least_cO2, i);
            if least_vals.len() == 1 {
                cO2 = Some(least_vals[0].clone());
            } else {
                cO2_bits = least_vals;
            }
        }
    }
    (oxygen.unwrap(), cO2.unwrap())
}

fn get_numbers_with_value(bits: Vec<Vec<usize>>, value: usize, index: usize) -> Vec<Vec<usize>> {
    bits.into_iter().filter(|b| b[index] == value).collect()
}

fn separate_bits(bit: &str) -> Vec<usize> {
    bit.split("")
        .filter(|&x| !x.is_empty())
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>()
}

fn most_and_least_common_bit_for_index(bits: &[Vec<usize>], index: usize) -> (usize, usize) {
    let sum: usize = bits.iter().map(|b| b[index]).sum();
    if sum > bits.len() / 2 {
        return (1, 0);
    }
    (0, 1)
}

fn most_and_least_common_bit_for_index_with_tiebreaker(
    bits: &[Vec<usize>],
    index: usize,
) -> (usize, usize) {
    let sum: usize = bits.iter().map(|b| b[index]).sum();
    if sum > bits.len() / 2 {
        return (1, 0);
    }
    if (sum as f32) == (bits.len() as f32 / 2.0 as f32) {
        return (1, 0);
    }
    (0, 1)
}

fn get_gamma_and_epsilon(bits: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>) {
    let mut gamma = vec![0; bits[0].len()];
    let mut epsilon = vec![0; bits[0].len()];
    for index in 0..bits[0].len() {
        let (most, least) = most_and_least_common_bit_for_index(bits, index);
        gamma[index] = most;
        epsilon[index] = least;
    }
    (gamma, epsilon)
}

fn get_decimal_rate_from_binary(rate: &[usize]) -> usize {
    let mut value = 0;
    for (index, bit) in rate.iter().enumerate() {
        value += bit << (rate.len() - 1 - index);
    }
    value
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn get_decimal() {
        let rate = [1, 0, 1, 1, 0];
        let calc = get_decimal_rate_from_binary(&rate.to_vec());
        assert_eq!(calc, 22);

        let rate = [0, 1, 0, 0, 1];
        let calc = get_decimal_rate_from_binary(&rate.to_vec());
        assert_eq!(calc, 9);
    }

    #[test]
    fn test_part1() {
        let input: Vec<Vec<usize>> =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
                .lines()
                .map(|line| line.trim())
                .map(|line| separate_bits(line))
                .collect();
        let power = part1(&input);
        assert_eq!(power, 198);
    }
}
