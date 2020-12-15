use std::collections::{HashMap, VecDeque};

fn main() {
    println!("{}", part1(vec![19, 0, 5, 1, 10, 13], 30000000));
}

fn part1(numbers: Vec<usize>, iterations: usize) -> usize {
    let mut last_spoken = HashMap::<usize, VecDeque<usize>>::new();

    for (i, num) in numbers.iter().enumerate() {
        last_spoken.insert(*num, VecDeque::from(vec![i + 1]));
    }

    let mut last_num = numbers[numbers.len() - 1];

    for i in numbers.len() + 1..iterations + 1 {
        last_num = match last_spoken.get(&last_num) {
            Some(vec) => {
                let mut curr_num = 0;
                if vec.len() > 1 {
                    let spoken_idxs = last_spoken.get(&last_num).unwrap();
                    curr_num =
                        spoken_idxs[spoken_idxs.len() - 1] - spoken_idxs[spoken_idxs.len() - 2];
                }
                curr_num
            }
            None => 0,
        };
        last_spoken
            .entry(last_num)
            .and_modify(|vec| vec.push_back(i))
            .or_insert_with(|| VecDeque::from(vec![i]));
        let queue = last_spoken.get_mut(&last_num).unwrap();
        if queue.len() > 2 {
            let _ = queue.pop_front();
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
