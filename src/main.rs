use std::env;


mod actions;

fn main() {
    println!(
        "Running in {}",
        env::current_dir().unwrap().to_str().unwrap()
    );
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("this needs to be told what to do");
        return;
    }

    match args[1].as_str() {
        "init" => actions::init::run(),
        "build" => actions::build::run(),
        "clean" => actions::clean::run(),
        _ => println!("Invalid option: {}, expected init or build", args[1])
    }
}
