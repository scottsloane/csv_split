# CSV_Split

csv_split 0.1.1

Scott Sloane

Split a csv into multiple files by line count.

USAGE:
    csv_split [OPTIONS] <SOURCE> [DESTINATION]

ARGS:
    <SOURCE>
            Path to the source CSV file

    <DESTINATION>
            Path to the output directory for the spit files

            [default: ./]

OPTIONS:
    -h, --help
            Print help information

    -r, --rows <ROWS>
            Max number of rows per file

            [default: 32000]

    -v, --verbose
            Verbose output (for debuging)

    -V, --version
            Print version information