use std::collections::HashMap;

fn main() {
    println!("{}", part1(vec![19, 0, 5, 1, 10, 13], 3000000));
}

fn part1(numbers: Vec<usize>, iterations: usize) -> usize {
    // let numbers: Vec<isize> = vec![19, 0, 5, 1, 10, 13];
    let mut last_spoken: HashMap<usize, Vec<usize>> = numbers
        .iter()
        .enumerate()
        .map(|(i, num)| (*num, vec![i + 1]))
        .collect();
    let mut last_num = numbers[numbers.len() - 1];

    for i in numbers.len() + 1..iterations + 1 {
        let spoken_before = match last_spoken.get(&last_num) {
            Some(vec) => vec.len() > 1,
            None => false,
        };
        if !spoken_before {
            last_spoken
                .entry(0)
                .and_modify(|vec| vec.push(i))
                .or_insert_with(|| vec![i]);
            last_num = 0;
        } else {
            let spoken_idxs = last_spoken.get(&last_num).unwrap();
            last_num = spoken_idxs[spoken_idxs.len() - 1] - spoken_idxs[spoken_idxs.len() - 2];
            last_spoken
                .entry(last_num)
                .and_modify(|vec| vec.push(i))
                .or_insert_with(|| vec![i]);
        }
    }
    last_num
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test1() {
        let numbers: Vec<usize> = vec![0, 3, 6];
        let result = part1(numbers);
        assert_eq!(result, 436);
    }
    #[test]
    fn test2() {
        let numbers: Vec<usize> = vec![1, 3, 2];
        let result = part1(numbers);
        assert_eq!(result, 1);
    }
    #[test]
    fn test3() {
        let numbers: Vec<usize> = vec![2, 1, 3];
        let result = part1(numbers);
        assert_eq!(result, 10);
    }
    #[test]
    fn test4() {
        let numbers: Vec<usize> = vec![1, 2, 3];
        let result = part1(numbers);
        assert_eq!(result, 27);
    }
    #[test]
    fn test5() {
        let numbers: Vec<usize> = vec![2, 3, 1];
        let result = part1(numbers);
        assert_eq!(result, 78);
    }
    #[test]
    fn test6() {
        let numbers: Vec<usize> = vec![3, 2, 1];
        let result = part1(numbers);
        assert_eq!(result, 438);
    }
    #[test]
    fn test7() {
        let numbers: Vec<usize> = vec![3, 1, 2];
        let result = part1(numbers);
        assert_eq!(result, 1836);
    }
}
