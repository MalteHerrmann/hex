fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_args = args.len();
    if n_args != 2 {
        panic!(
            "wrong number of input arguments; expected: 2; got: {}",
            n_args
        );
    }

    match args[1].strip_prefix("0x") {
        Some(s) => {
            let number =
                u32::from_str_radix(s, 16).expect("failed to parse u32 from input argument");
            println!("{}", number);
        }
        None => panic!("expected hex number to be passed; no 0x prefix found"),
    }
}
