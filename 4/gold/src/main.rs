fn main() {
    let input = "ckczppom";

    for i in 0..std::u32::MAX {
        let input = input.to_owned() + &i.to_string();
        let digest = md5::compute(&input);

        if digest.0[0] == 0 && digest.0[1] == 0 && digest.0[2] == 0 {
            println!("{}", input);
            break;
        }
    }
}
