enum Op {
    Add,
    Multiply,
}

fn check(target: u64, components: &Vec<u64>, i: usize, sub_result: u64, op: Op) -> bool {
    let component = components[i];
    let sub_r = match op {
        Op::Add => sub_result
            .checked_add(component)
            .expect("No overflow while adding"),
        Op::Multiply => sub_result
            .checked_mul(component)
            .expect("No overflow while multiplying"),
    };

    if sub_r == target && i + 1 >= components.len() {
        // We used all numbers and the result checks out
        return true;
    } else if sub_r > target {
        // We overshot, no point in going further
        return false;
    } else if i + 1 >= components.len() {
        // We reached the end but results don't match
        return false;
    } else {
        return check(target, components, i + 1, sub_r, Op::Add)
            || check(target, components, i + 1, sub_r, Op::Multiply);
    }
}

pub fn has_solution(total: u64, components: &Vec<u64>) -> bool {
    let first_number = components[0];

    check(total, components, 1, first_number, Op::Add)
        || check(total, components, 1, first_number, Op::Multiply)
}

pub fn process(input: &str) -> u64 {
    let equations = super::parse(input);

    let result: u64 = equations
        .into_iter()
        .filter(|(total, comp)| has_solution(*total, comp))
        .map(|(total, _)| total)
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let result = process(input);

        assert_eq!(3749, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(850435817339, result);
    }
}
