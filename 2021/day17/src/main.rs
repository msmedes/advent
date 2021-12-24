use std::num;
use std::ops::Range;
fn main() {
    // input = target area: x=269..292, y=-68..-44
    let target_area = TargetArea {
        x1: 269,
        x2: 292,
        y1: -68,
        y2: -44,
    };
    part2(target_area);
}

fn part2(area: TargetArea) {
    let lowest_x = area.lowest_x();
    let x_ranges: Vec<(i64, i64, i64)> = (lowest_x..=area.x2)
        .map(|x| area.steps_inbound_for_x(x))
        .flatten()
        .collect();
    let pairs: usize = area
        .y_range()
        .map(|y| area.steps_inbound_for_y(y, &x_ranges))
        .sum();

    dbg!(pairs);
}

#[derive(Debug)]
struct TargetArea {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

impl TargetArea {
    fn y_range(&self) -> Range<i64> {
        let min_y = std::cmp::min(self.y1, self.y2);
        min_y..(min_y.abs())
    }

    fn triangular_num(&self, x: i64) -> i64 {
        (x * (x + 1)) / 2
    }
    fn lowest_x(&self) -> i64 {
        let mut val = 1;
        while self.triangular_num(val) < std::cmp::min(self.x1, self.x2) {
            val += 1;
        }
        val
    }

    fn steps_inbound_for_x(&self, x: i64) -> Option<(i64, i64, i64)> {
        let min_x = std::cmp::min(self.x1, self.x2);
        let max_x = std::cmp::max(self.x1, self.x2);
        let mut steps = 0;
        let mut position = 0;
        let mut velocity = x;
        let mut min_steps = i64::MAX;
        let mut max_steps = i64::MIN;

        while position < min_x {
            steps += 1;
            position += velocity;
            velocity -= 1;
        }
        while position <= max_x && velocity > -1 {
            min_steps = std::cmp::min(min_steps, steps);
            max_steps = std::cmp::max(max_steps, steps);
            steps += 1;
            position += velocity;
            velocity -= 1;
        }
        if velocity == -1 {
            max_steps = i64::MAX;
        }
        if min_steps == i64::MAX && max_steps == i64::MIN {
            None
        } else {
            Some((min_steps, max_steps, x))
        }
    }

    fn steps_inbound_for_y(&self, y: i64, x_ranges: &[(i64, i64, i64)]) -> usize {
        let mut count = 0;

        for x_range in x_ranges {
            let mut velocity = y;
            let mut position = 0;
            let max_y = std::cmp::max(self.y1, self.y2);
            let min_y = std::cmp::min(self.y1, self.y2);
            let mut steps = 0;

            if y < 1 {
                while position >= min_y && steps <= x_range.1 {
                    if position >= min_y && position <= max_y && steps >= x_range.0 {
                        count += 1;
                        break;
                    }
                    position += velocity;
                    velocity -= 1;
                    steps += 1;
                }
            } else if (x_range.1 - (y + 1)) >= 0 {
                let max_height = self.triangular_num(y);
                let min_steps = x_range.0 - (y + 1);
                let max_steps = x_range.1 - (y + 1);
                let min_y_position_delta = max_height - max_y;
                let max_y_position_delta = max_height - min_y;
                for i in min_steps..=max_steps {
                    let tri = self.triangular_num(i);
                    if min_y_position_delta <= tri && tri <= max_y_position_delta {
                        count += 1;
                        break;
                    }
                    if tri > max_y_position_delta {
                        break;
                    }
                }
            }
        }
        count
    }
}
