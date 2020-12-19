fn main() {
    let lines = read_file("input.txt");
    let result: usize = lines.iter().map(|line| solve2(line)).sum();
    println!("{}", result);
}

#[derive(Debug, PartialEq)]
enum Operator {
    Mult,
    Add,
    None,
}

fn solve2(math: &str) -> usize {
    let (result, _) = helper2(math, 0);
    result
}

fn helper2(math: &str, start: usize) -> (usize, usize) {
    let mut index = start;
    let mut result = 0;
    let mut curr_operator = Operator::None;
    let mut current_value: Option<usize> = Some(0);

    while index < math.len() {
        let this_char = &math[index..index + 1];
        match this_char {
            "+" => {
                curr_operator = Operator::Add;
                current_value = None;
                index += 1
            }
            "*" => {
                curr_operator = Operator::Mult;
                let (eval, next) = helper2(math, index + 1);
                current_value = Some(eval);
                index = next;
            }
            "(" => {
                let (eval, next) = helper2(math, index + 1);
                current_value = Some(eval);
                index = next;
            }
            ")" => {
                return (result, index + 1);
            }
            " " => {
                index += 1;
                continue;
            }
            _ => {
                current_value = Some((this_char).parse::<usize>().unwrap());
                index += 1
            }
        }
        result = match (&curr_operator, current_value) {
            (Operator::Add, Some(current_value)) => result + current_value,
            (Operator::Mult, Some(current_value)) => result * current_value,
            (Operator::None, Some(current_value)) => current_value,
            (_, None) => result,
        };
        if curr_operator == Operator::Mult {
            break;
        }
    }
    (result, index)
}

// fn solve(math: &str) -> usize {
//     let (result, _) = helper(math, 0);
//     result
// }

// fn helper(math: &str, start: usize) -> (usize, usize) {
//     let mut index = start;
//     let mut result = 0;
//     let mut curr_operator = Operator::None;
//     let mut current_value: Option<usize> = Some(0);

//     while index < math.len() {
//         match &math[index..index + 1] {
//             "+" => {
//                 curr_operator = Operator::Add;
//                 current_value = None;
//             }
//             "*" => {
//                 curr_operator = Operator::Mult;
//                 current_value = None;
//             }
//             "(" => {
//                 let (eval, next) = helper(math, index + 1);
//                 current_value = Some(eval);
//                 index = next;
//             }
//             ")" => return (result, index),
//             " " => {
//                 index += 1;
//                 continue;
//             }
//             _ => current_value = Some((&math[index..index + 1]).parse::<usize>().unwrap()),
//         }

//         result = match (&curr_operator, current_value) {
//             (Operator::Add, Some(current_value)) => result + current_value,
//             (Operator::Mult, Some(current_value)) => result * current_value,
//             (Operator::None, Some(current_value)) => current_value,
//             (_, None) => result,
//         };
//         index += 1;
//     }
//     (result, index)
// }

fn read_file(filename: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(filename).unwrap();
    contents
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    // #[test]
    // fn test1() {
    //     let expected = 71;
    //     let actual = solve("1 + 2 * 3 + 4 * 5 + 6");
    //     assert_eq!(actual, expected);
    // }
    // #[test]
    // fn test2() {
    //     let expected = 51;
    //     let actual = solve("1 + (2 * 3) + (4 * (5 + 6))");
    //     assert_eq!(actual, expected);
    // }
    // #[test]
    // fn test3() {
    //     let expected = 26;
    //     let actual = solve("2 * 3 + (4 * 5)");
    //     assert_eq!(actual, expected);
    // }
    // #[test]
    // fn test4() {
    //     let expected = 437;
    //     let actual = solve("5 + (8 * 3 + 9 + 3 * 4 * 3)");
    //     assert_eq!(actual, expected);
    // }
    // #[test]
    // fn test5() {
    //     let expected = 12240;
    //     let actual = solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    //     assert_eq!(actual, expected);
    // }
    // #[test]
    // fn test6() {
    //     let expected = 13632;
    //     let actual = solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn test7() {
        let expected = 231;
        let actual = solve2("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test8() {
        let expected = 51;
        let actual = solve2("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test9() {
        let expected = 46;
        let actual = solve2("2 * 3 + (4 * 5)");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test10() {
        let expected = 1445;
        let actual = solve2("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test11() {
        let expected = 669060;
        let actual = solve2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test12() {
        let expected = 23340;
        let actual = solve2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(actual, expected);
    }
}
