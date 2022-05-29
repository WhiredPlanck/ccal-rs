use clap::Parser;

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcmds: Option<SubCommands>,
    /// Specify a certain month (range: 1 - 12)
    pub month: Option<String>,
    /// Speicy a certain year (range: 1645 - 7000
    pub year: Option<String>,
}

#[derive(Parser)]
pub enum SubCommands {
    /// Generate HTML table output
    #[clap(display_order = 1)]
    #[clap(short_flag = 't')]
    Html,
    /// Generate encapsulated PostScript output
    #[clap(display_order = 2)]
    #[clap(short_flag = 'p')]
    Postscript,
    /// Generate XML output
    #[clap(display_order = 3)]
    #[clap(short_flag = 'x')]
    Xml,
    /// Generate output in Simplified Chinese (in UTF-8 by default)
    #[clap(display_order = 4)]
    #[clap(short_flag = 'g', alias = "simp")]
    Simplified,
    /// Generate output in Traditional Chinese (in UTF-8 by default)
    #[clap(display_order = 5)]
    #[clap(short_flag = 'b', alias = "trad")]
    Traditional,
}
