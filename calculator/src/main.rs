fn main() {
    if let Err(e) = calculator::get_args().and_then(calculator::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
