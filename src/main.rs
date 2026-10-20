use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: {} <unpacked_menu_dir> [<output_dir>]", args[0]);
        std::process::exit(1);
    }

    let input = PathBuf::from(&args[1]);

    let output = if args.len() == 2 {
        env::current_exe().unwrap()
            .parent().unwrap()
            .join("images")
        
    } else {
        PathBuf::from(&args[2])
    };
    
    unsafe { 
        Ferrisouls::textures::dump_tgas(
            &input,
            &output
        ).unwrap()
    }
}