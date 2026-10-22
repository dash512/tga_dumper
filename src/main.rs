use std::env;
use std::path::{Path, PathBuf};

//Code from Ferrisouls::textures::dump_tgas
unsafe fn process(menu_path: &Path, output_dir: &Path, sprites: bool) -> Result<(), Ferrisouls::errors::FerrisoulsError> {
    use Ferrisouls::{binary::IO, binders::{Binder, MetaBinder}};
    use Ferrisouls::formats::{tpf::{TPF, TPFTexture}, sblyt::{Layout, LayoutBinder}};
    use Ferrisouls::textures::image::{self, ImageFormat};
    use Ferrisouls::files;

    unsafe { Ferrisouls::oodle::core::oodle_auto().unwrap(); }

    std::fs::create_dir_all(output_dir)?;

    let tpfs = files::get_files_by(
        menu_path.to_path_buf(), 
        |s| s.ends_with(".tpf") || s.ends_with(".tpf.dcx")
    );
    let sblyts = files::get_files_by(
        menu_path.to_path_buf(), 
        |s| s.contains(".sblytbnd")
    );

    let mut textures: Vec<TPFTexture> = Vec::new();
    let mut layouts: Vec<Layout> = Vec::new();

    {
        for tpf_path in tpfs {
            let (tpf, _) = unsafe { TPF::unpack(&tpf_path)? };
            println!("Unpacked TPF: {:?} with {} entries", tpf_path.file_name(), tpf.count());
            textures.extend(tpf.textures);
        }

        for sblyt_path in sblyts {
            let (sblyt, _) = unsafe { LayoutBinder::unpack_binder(&sblyt_path)? };
            println!("Unpacked Layout Binder: {:?} with {} entries", sblyt_path.file_name(), sblyt.count());
            layouts.extend(sblyt.entries);
        }
    }

    for tex in textures {
        let tga = tex.to(ImageFormat::TGA)?;
        std::fs::write(
            output_dir.join(format!("{}.tga", tex.name)),
            &tga,
        )?;
        println!("Wrote TPFTexture: {:?}.", tex.name);

        if !do_sprites {
            continue;
        }

        let layout = layouts
            .iter()
            .find(|l| l.stem() == tex.name);

        if let Some(layout) = layout {
            println!("Found layout for {}! Cropping {} subtextures...", tex.name, layout.entries.len());

            for sub in layout.iter() {
                let sprite = image::crop_tga(
                    &tga, 
                    sub.x.into(), 
                    sub.y.into(), 
                    sub.width.into(), 
                    sub.height.into()
                );
                match sprite {
                    None => {
                        println!("Failed to crop tga `{}`, skipping!", tex.name);
                        continue;
                    },
                    Some(s) => {
                        std::fs::write(
                            output_dir.join(format!("{}.tga", sub.name.replace(".png", ""))),
                            &s,
                        )?;
                    }
                }
            }
        }
    }
    
    Ok(())
}


fn main() {
    let mut args = env::args().skip(1);

    let mut sprites = false;
    let mut positional: Vec<PathBuf> = Vec::new();

    while let Some(arg) = args.next() {
        if let Some(value) = arg.strip_prefix("--sprites=") {
            sprites = match value {
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
        process(
            input_path,
            output_path,
            sprites
        ).unwrap()
    }

}