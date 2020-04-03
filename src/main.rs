use rand::Rng;
fn main() {
    let motivate = [
    "Do it",
    "Don't let your dreams be dreams",
    "Just do it",
    "If you're tired of starting over, Stop giving up",
    "Make your dreams come true",
    "Nothing is impossible",
    "No, what are you waiting for?",
    "Some people dream of success, While you're gonna wake up and work hard at it",
    "You should get to the point\nWhere anyone else would quit\nAnd you're not going to stop there",
    "Yes you can",
    "Yesterday you said tomorrow"
];
    let mut rng = rand::thread_rng();
    if motivate.is_empty() {
        println!("Out of Energy!");
    } else {
        println!("{}", motivate[rng.gen_range(0, motivate.len())]);
    }
}
