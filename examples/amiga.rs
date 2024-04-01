use clap::Parser;
use xmrs::amiga::amiga_module::AmigaModule;

#[derive(Parser)]
struct Cli {
    #[arg(
        short = 'f',
        long,
        default_value = "/home/user/Downloads/ultimate_mod_collection[1433mods][2021-07-19]_compiled_by_spacedrone808/breaks/astral_projection.mod",
        value_name = "filename"
    )]
    filename: Option<String>,

    /// Turn pattern informations on
    #[arg(short = 'p', long, default_value = "false")]
    patterns: bool,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match cli.filename {
        Some(filename) => {
            println!("--===~ XmRs Amiga Module Info Example ~===--");
            println!("(c) 2024 Sébastien Béchet\n");
            println!("opening {}", filename);
            let contents = std::fs::read(filename.trim())?;
            match AmigaModule::load(&contents) {
                Ok(amiga) => {
                    // println!("{:?}", amiga);
                    let module = amiga.to_module();
                    println!("{:?}", module);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        _ => {}
    }
    Ok(())
}
