#[derive(PartialEq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    let lines = text.split('\n');

    let mut lights: [u8; 1000000] = [0; 1_000_000];

    for line in lines {
        let mut line = line.split(' ');

        let action = match line.next() {
            Some("turn") => match line.next() {
                Some("on") => Action::TurnOn,
                Some("off") => Action::TurnOff,
                _ => unreachable!(),
            },
            Some("toggle") => Action::Toggle,
            _ => break,
        };

        let mut start = line.next().unwrap().split(',');
        let start_x = start.next().unwrap().parse::<u32>().unwrap();
        let start_y = start.next().unwrap().parse::<u32>().unwrap();

        line.next();

        let mut end = line.next().unwrap().split(',');
        let end_x = end.next().unwrap().parse::<u32>().unwrap();
        let end_y = end.next().unwrap().parse::<u32>().unwrap();

        assert!(start_x <= end_x);
        assert!(start_y <= end_y);

        for y in start_y..=end_y {
            for x in start_x..=end_x {
                match action {
                    Action::TurnOn => lights[(y * 1000 + x) as usize] += 1,
                    Action::TurnOff => {
                        lights[(y * 1000 + x) as usize] =
                            lights[(y * 1000 + x) as usize].saturating_sub(1)
                    }
                    Action::Toggle => lights[(y * 1000 + x) as usize] += 2,
                };
            }
        }
    }

    let mut light_count: u32 = 0;

    for light in lights.iter() {
        light_count += *light as u32;
    }

    println!("{}", light_count);
}
