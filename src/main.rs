#[cfg(not(target_arch = "wasm32"))]
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The file to open. Set it to '-' to read from stdin.
    /// If not set, this reads from stdin.
    file: Option<String>,
    /// The file to output to. Set it to '-' to write to stdout.
    /// If not set, this writes to a kdl file with the same name as the input file,
    /// or 'output.kdl' if reading from stdin.
    output: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use clap::Parser;
    use oudia_to_kdl::convert;
    use std::io::Read;
    let cli = Cli::parse();
    let input = match cli.file {
        Some(ref filename) if filename != "-" => {
            std::fs::read_to_string(filename).expect("Failed to read the input file")
        }
        _ => {
            let mut buffer = String::new();
            println!("Reading from stdin...");
            std::io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            buffer
        }
    };
    let output = convert(&input).expect("Failed to parse OUD2 file");
    match cli.output {
        Some(filename) => {
            if filename != "-" {
                std::fs::write(filename, output).expect("Failed to write to the output file")
            } else {
                println!("{}", output);
            }
        }
        None => {
            let output_filename = match cli.file {
                Some(ref filename) if filename != "-" => {
                    let mut path = std::path::PathBuf::from(filename);
                    path.set_extension("kdl");
                    path.to_string_lossy().to_string()
                }
                _ => "output.kdl".to_string(),
            };
            std::fs::write(&output_filename, output).expect("Failed to write to the output file");
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // No-op for wasm32 target
}
