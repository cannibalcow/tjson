pub mod args {
    use clap::{arg, Parser};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct TJsonArgs {
        #[arg(short, long, help = "Path to in json struct. Eg, /path/to/json/node")]
        pub pointers: Vec<String>,

        #[arg(short, long, help = "Http json resource, if not given read stdin")]
        pub source: String,

        #[arg(
            short = 'i',
            long = "polling-intervall",
            default_value_t = 3,
            help = "Polling interval in seconds"
        )]
        pub polling_interval: usize,
    }
}
