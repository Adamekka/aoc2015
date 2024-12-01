use std::cmp::min;

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();
    let lines = text.split('\n');

    let mut result: u32 = 0;

    for line in lines {
        let mut c = line.split('x');

        if line.is_empty() {
            break;
        }

        let l = c.next().unwrap().parse::<u32>().unwrap();
        let w = c.next().unwrap().parse::<u32>().unwrap();
        let h = c.next().unwrap().parse::<u32>().unwrap();

        let side1 = l * w;
        let side2 = w * h;
        let side3 = h * l;

        let smallest_side = min(side1, min(side2, side3));

        result += 2 * (side1 + side2 + side3) + smallest_side;
    }

    println!("{}", result);
}
