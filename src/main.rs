fn main() {
    // \ args
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    let ini = if argc > 1 { &argv[1] } else { "lib/ini.ini" };
    let src = std::fs::read_to_string(ini).unwrap();
    println!("#{:?} {:?} -> {:?}", argc, argv, ini);
    // / args
    println!("\n{:?}\n", src);
}
