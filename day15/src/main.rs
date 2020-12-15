use std::collections::HashMap;

fn main() {
    println!("{}", part1(vec![19, 0, 5, 1, 10, 13], 30000000));
}

fn part1(numbers: Vec<usize>, iterations: usize) -> usize {
    let mut last_spoken = HashMap::<usize, usize>::new();

    for (i, num) in numbers.iter().enumerate() {
        last_spoken.insert(*num, i);
    }

    let mut last_num = numbers[numbers.len() - 1];

    for i in numbers.len()..iterations {
        let curr_num = match last_spoken.get(&last_num) {
            Some(last) => i - last,
            None => 0,
        };

        last_spoken.insert(last_num, i);
        last_num = curr_num;
    }
    last_num
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test1() {
        let numbers: Vec<usize> = vec![0, 3, 6];
        let result = part1(numbers, 2020);
        assert_eq!(result, 436);
    }
    #[test]
    fn test2() {
        let numbers: Vec<usize> = vec![1, 3, 2];
        let result = part1(numbers, 2020);
        assert_eq!(result, 1);
    }
    #[test]
    fn test3() {
        let numbers: Vec<usize> = vec![2, 1, 3];
        let result = part1(numbers, 2020);
        assert_eq!(result, 10);
    }
    #[test]
    fn test4() {
        let numbers: Vec<usize> = vec![1, 2, 3];
        let result = part1(numbers, 2020);
        assert_eq!(result, 27);
    }
    #[test]
    fn test5() {
        let numbers: Vec<usize> = vec![2, 3, 1];
        let result = part1(numbers, 2020);
        assert_eq!(result, 78);
    }
    #[test]
    fn test6() {
        let numbers: Vec<usize> = vec![3, 2, 1];
        let result = part1(numbers, 2020);
        assert_eq!(result, 438);
    }
    #[test]
    fn test7() {
        let numbers: Vec<usize> = vec![3, 1, 2];
        let result = part1(numbers, 2020);
        assert_eq!(result, 1836);
    }
}
