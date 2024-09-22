use clap::Parser;
use xmrs::s3m::s3m_module::S3mModule;

#[derive(Parser)]
struct Cli {
    #[arg(
        short = 'f',
        long,
        default_value = "PANIC.S3M",
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
            println!("--===~ XmRs S3M Module Info Example ~===--");
            println!("(c) 2024 Sébastien Béchet\n");
            println!("opening {}", filename);
            let contents = std::fs::read(filename.trim())?;
            match S3mModule::load(&contents) {
                Ok(s3m) => {
                    // println!("{:?}", s3m);
                    let module = s3m.to_module();
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
