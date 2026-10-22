use std::env;
use std::path::PathBuf;

fn main() {
    let mut args = env::args().skip(1);

    let mut do_sprites = false;
    let mut positional: Vec<PathBuf> = Vec::new();

    while let Some(arg) = args.next() {
        if let Some(value) = arg.strip_prefix("--sprites=") {
            do_sprites = match value {
                "yes" => true,
                "no" => false,
                _ => {
                    eprintln!("Invalid mode: {value}. Expected one of: [yes|no]");
                    std::process::exit(1);
                }
            };
        } else if arg.starts_with('-') {
            eprintln!("Unknown argument: {arg}");
            std::process::exit(1);
        } else {
            positional.push(PathBuf::from(arg));
        }
    }

    if positional.len() == 0 { //no paths
        eprintln!("Usage: program <input> <output> --sprites=yes|no");
        std::process::exit(1);
    }

    let input_path = &positional[0];
    let output_path = if positional.len() < 2 {
        &env::current_exe().unwrap()
            .parent().unwrap()
            .join("images")
        
    } else {
        &positional[1]
    };

    println!("-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-\nBegin Processing...\n-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-");

    unsafe {
        Ferrisouls::textures::dump_tgas(
            input_path,
            output_path,
            do_sprites
        ).unwrap()
    }

}