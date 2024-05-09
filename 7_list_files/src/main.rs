use glob::glob;
use std::error::Error;

use std::fs::File;
use std::io::prelude::*;

use clap::Parser;
use clap_verbosity_flag::{ErrorLevel, Verbosity};
use log::{debug, error, info};
use shells::sh;
use std::path::PathBuf;

mod logging;
use logging::{init_logger, LogExt};

fn process_file(file: &mut File, path: PathBuf) -> Result<(), Box<dyn Error>> {
    info!("processing_file: {:?}", path.display());
    let (code, content, stderr) = sh!("cat {:?}", path.display());
    if code != 0 {
        error!("Error {code}: {stderr} (Error 27279-8684-21823)");
    } else {
        debug!("File {}: {content}", path.display());
        let _ = write!(file, "{content}");
    }
    Ok(())
}

fn process_pattern(file: &mut File, pattern: &str) -> Result<(), Box<dyn Error>> {
    // write!(&mut writer, "Factorial of {} = {}", num, factorial);

    for entry in glob(pattern).log_msg("Failed to read glob pattern")? {
        match entry {
            Ok(path) => process_file(file, path)?,
            Err(e) => Err(e).log_msg("ERROR: processing pattern")?,
        }
    }
    Ok(())
}

#[derive(Parser, Debug, Clone)]
#[clap(
    author = "Christof Fetzer",
    version = "0.2.0",
    about = "Display hash of a list of files.",
    long_about = "Files can be defined as follows; ..."
)]
struct Args {
    /// Set the verbosity level.
    /// Add more "v"s to increase the verbosity level.
    #[clap(flatten)]
    verbose: Verbosity<ErrorLevel>,

    /// Define the mode, i.e., if the application should run in "production" or in "debug" mode.
    /// By default, we use "production" mode.
    #[clap(short, long, default_value = "x/vault*")]
    pattern: String,

    /// Image name
    #[clap(short, long, default_value = "x/vault*")]
    image: String,

    /// File name
    #[clap(short, long, default_value = "config_framgment.yaml")]
    filename: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    init_logger(&args.verbose);
    info!("List files: Arguments= {:?}", args);
    let mut file = File::create(&args.filename).log_msg("")?;

    let _ = process_pattern(&mut file, &args.pattern).log_msg(&format!(
        "Error processing pattern {} 21076-25241-23702",
        args.pattern
    ));
    Ok(())
}
