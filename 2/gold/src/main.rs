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

        let mut dimensions = [l, w, h];
        dimensions.sort();

        let smallest_dimension = dimensions[0];
        let second_smallest_dimension = dimensions[1];

        let ribbon = smallest_dimension
            + smallest_dimension
            + second_smallest_dimension
            + second_smallest_dimension;

        let bow = l * w * h;

        result += ribbon + bow;
    }

    println!("{}", result);
}
