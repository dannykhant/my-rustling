use clap::Parser;
use std::fs;

#[derive(clap::ValueEnum, Clone, Debug)]
enum SizeFormat {
    Mb,
    Gb,
    Kb
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    file_only: bool,

    #[arg(short, long)]
    size_format: SizeFormat,

    #[arg(short, long)]
    precision: bool
}

fn main() {
    let args = Cli::parse();
    let precision = if args.precision {9} else {2};

    let mut total_size: f64 = 0.;
    let mut sizes = Vec::new();
    
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();

        if args.file_only && meta.is_dir() {
            continue;
        }

        total_size += meta.len() as f64;

        match args.size_format {
            SizeFormat::Kb => {
                sizes.push((entry.file_name(), meta.len() as f64 / 1024.0));
            },
            SizeFormat::Mb => {
                sizes.push((entry.file_name(), meta.len() as f64 / (1024.0 * 1024.0)));
            },
            SizeFormat::Gb => {
                sizes.push((entry.file_name(), meta.len() as f64 / (1024.0 * 1024.0 * 1024.0)));
            },
        }
    }

    for (name, size) in sizes {
        let size_percent = (size / total_size) * 100.0; 
        let size_format = match args.size_format {
            SizeFormat::Kb => "KB",
            SizeFormat::Mb => "MB",
            SizeFormat::Gb => "GB",
        };

        println!("{:<20}: {:>10.prec$} {} | {:>10.prec$} %", 
            name.to_string_lossy(), size, size_format, size_percent, prec = precision);
    }

    match args.size_format {
        SizeFormat::Mb => {
            println!("Total Size: {:.prec$} MB", total_size / (1024.0 * 1024.0), prec = precision);
        },
        SizeFormat::Gb => {
            println!("Total Size: {:.prec$} GB", total_size / (1024.0 * 1024.0 * 1024.0), prec = precision);
        },
        SizeFormat::Kb => {
            println!("Total Size: {:.prec$} KB", total_size / 1024.0, prec = precision);
        },
    }
}
