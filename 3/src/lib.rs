use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Coord(i64, i64);

pub fn visit_count(plan: String) -> usize {
    list_locatoins(plan).len()
}

pub fn visit_robot_count(plan: String) -> usize {
    // split the plan
    let mut santa_plan = String::new();
    let mut robot_plan = String::new();

    for (i, c) in plan.chars().enumerate() {
        match i % 2 {
            0 => santa_plan.push(c),
            1 => robot_plan.push(c),
            _ => panic!("math"),
        }
    }

    list_locatoins(santa_plan)
        .union(&list_locatoins(robot_plan))
        .count()
}

fn list_locatoins(plan: String) -> HashSet<Coord> {
    let mut visted = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    visted.insert(Coord(x, y));

    for c in plan.chars() {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => panic!("unknown char {}", c),
        }
        visted.insert(Coord(x, y));
        //println!("add {}/{}", x, y);
    }

    visted
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(visit_count("".to_string()), 1);
    }

    #[test]
    fn east() {
        assert_eq!(visit_count(">".to_string()), 2);
    }

    #[test]
    fn west() {
        assert_eq!(visit_count("<".to_string()), 2);
    }

    #[test]
    fn north() {
        assert_eq!(visit_count("^".to_string()), 2);
    }

    #[test]
    fn south() {
        assert_eq!(visit_count("v".to_string()), 2);
    }

    #[test]
    fn square() {
        assert_eq!(visit_count("v>^<".to_string()), 4);
    }

    #[test]
    fn two_bounce() {
        assert_eq!(visit_count("><><><><><>".to_string()), 2);
    }

    #[test]
    fn visit_robot_updown() {
        assert_eq!(visit_robot_count("^v".to_string()), 3);
    }

    #[test]
    fn visit_robot_square() {
        assert_eq!(visit_robot_count("^>v<".to_string()), 3);
    }

    #[test]
    fn visit_robot_bounce() {
        assert_eq!(visit_robot_count("^v^v^v^v^v".to_string()), 11);
    }
}
