pub mod args {
    use clap::{arg, Parser};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct TJsonArgs {
        #[arg(short, long)]
        pub pointers: Vec<String>,

        #[arg(short, long)]
        pub source: String,
    }
}
