const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let guesser = roget::algorithms::Naive::new();
    for answer in GAMES.split_whitespace() {
        roget::play(answer, guesser)
    }
}
