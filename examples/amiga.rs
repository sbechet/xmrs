use clap::Parser;
use xmrs::amiga::amiga_module::AmigaModule;
use xmrs::xm::xmmodule::XmModule;

#[derive(Parser)]
struct Cli {
    #[arg(
        short = 'f',
        long,
        default_value = "example.mod",
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
                    // let mut xm = XmModule::from_module(&module);
                    // let xmfile = xm.save().unwrap();
                    // std::fs::write("/home/user/output.xm", xmfile);
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
