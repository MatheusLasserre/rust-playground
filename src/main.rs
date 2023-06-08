

fn main() {
    std::fs::read_to_string("lines")
    .unwrap()
    .lines()
    .enumerate()
    .filter(|(idx, _)| idx % 2 == 0)
    .skip(2)
    .take(2)
    .for_each(|(_, str)| println!("{}", str));
    

}