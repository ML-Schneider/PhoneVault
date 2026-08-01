use std::path::PathBuf;

use phonevault_core::transfer::preservation::PreservationJob;

use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(
    name = "phonevault",
    version = "0.1.0",
    about = "Digital memory preservation system"
)]
struct Cli {

    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {

    Init {
        path: String,
    },

    Scan {
        path: String,
    },

    Preserve {
        source: String,
        destination: String,
    },

    Verify {
        path: String,
    },
}


fn main() {

    let cli =
        Cli::parse();


    match cli.command {

        Commands::Init { path } => {

            println!(
                "Initializing vault at {}",
                path
            );

        }


        Commands::Scan { path } => {

            println!(
                "Scanning {}",
                path
            );

        }


        Commands::Preserve {
    source,
    destination,
} => {

    println!(
        "Starting preservation..."
    );


    let job =
        PreservationJob::new(
            PathBuf::from(source),
            PathBuf::from(destination),
        );


    let report =
        job.execute();


    println!();

    println!("Preservation complete");

    println!(
        "Files scanned: {}",
        report.files_scanned
    );

    println!(
        "Files copied: {}",
        report.files_copied
    );

    println!(
        "Files verified: {}",
        report.files_verified
    );

    println!(
        "Failures: {}",
        report.failures
    );
}


        Commands::Verify { path } => {

            println!(
                "Verifying {}",
                path
            );

        }
    }
}