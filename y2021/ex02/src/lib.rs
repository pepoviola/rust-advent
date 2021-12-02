#[derive(Debug, Clone)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_line(line: &str) -> Command {
    let mut parts = line.split(' ');
    let cmd = parts.next().expect("Instruction not found");
    let val: i32 = parts
        .next()
        .expect("Value not found")
        .parse()
        .expect("Cannot convert value to i32");

    match cmd {
        "forward" => Command::Forward(val),
        "down" => Command::Down(val),
        "up" => Command::Up(val),
        x => panic!("Unknown instruction: {}", x),
    }
}

pub fn part1(input: &str) -> i32 {
    let mut depth = 0;
    let mut horizontal_pos = 0;
    let cmds: Vec<Command> = input.lines().map(parse_line).collect();
    for cmd in cmds {
        match cmd {
            Command::Forward(x) => horizontal_pos += x,
            Command::Up(x) => depth -= x,
            Command::Down(x) => depth += x,
        }
    }

    horizontal_pos * depth
}

pub fn part2(input: &str) -> i32 {
    let mut depth = 0;
    let mut horizontal_pos = 0;
    let mut aim = 0;
    let cmds: Vec<Command> = input.lines().map(parse_line).collect();
    for cmd in cmds {
        match cmd {
            Command::Forward(x) => {
                horizontal_pos += x;
                depth += x * aim;
            }
            Command::Up(x) => aim -= x,
            Command::Down(x) => aim += x,
        }
    }

    horizontal_pos * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(part1(input), 150);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 2039912);
    }

    #[test]
    fn test_part2_example() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(part2(input), 900);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1942068080);
    }
}
