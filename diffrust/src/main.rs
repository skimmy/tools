fn main() {
    let config = diffrust::args::Config::build();

    if let Err(e) = diffrust::run(config) {
        println!("Error\n  {e}\nTerminating!");
        std::process::exit(1)
    }
}
