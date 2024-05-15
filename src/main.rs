use gh_actions_test_rs::greet;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Error: invalid args");
    }

    println!("{}", greet(&args[1]));
}
