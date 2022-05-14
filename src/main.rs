use std::error::Error;
// use std::io;
use std::process;
use std::path::Path;
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author="Scott Sloane", version, about = "Split a CSV", long_about = "Split a csv into multiple files by line count.")]
struct Args {
    
    /// Max number of rows per file
    #[clap(short, long, default_value_t = 32000)]
    rows: u32,

    /// Verbose output (for debuging)
    #[clap(short, long)]
    verbose: bool,

    /// Path to the source CSV file
    #[clap()]
    source: String,

    /// Path to the output directory for the spit files
    #[clap(default_value="./")]
    destination: String,

}

fn write_to_file(data: &[csv::StringRecord], filename: String) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(filename)?;

    for row in data {
        writer.write_record(row.iter())?;
    }

    // wtr.write_record(record.iter())?;
    writer.flush()?;
    Ok(())
}

fn split(rows: u32, verbose: bool, source: String, destination: String) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.

    let path = Path::new(&source);
    
    let mut i:u32 = 0;
    let mut page:u32 = 0;
    let mut data:Vec<csv::StringRecord> = Vec::new();
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path(&source)?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        if verbose {
            println!("{:?}", record);
        }
        data.push(record);
        i+=1;
        if i >= rows {
            // Flush and Save current File
            
            // This will fuck up if destination does not end with /
            write_to_file(data.as_slice(), format!("{}{}{}.csv", destination, path.file_stem().unwrap_or_default().to_str().unwrap(), page))?;
            //Start new File
            data = Vec::new();

            // Reset i
            i = 0;
            page += 1;
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    let _res = split(args.rows, args.verbose, args.source, args.destination);
    // if let Err(err) = example(args.rows, args.verbose, args.source, args.destination) {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }
    process::exit(0);
}