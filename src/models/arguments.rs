use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Number of rows to show
    #[arg(short = 'n', long, default_value_t = 1000)]
    pub count: u16,

    /// Should the world stats be excluded
    #[arg(short, long, default_value_t = false)]
    pub world_stats: bool,

    /// Show full or only totals [default: false]
    #[arg(short = 'f', long, default_value_t = false)]
    pub show_full: bool,
}

