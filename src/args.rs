use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub manga: String,

    #[arg(short, long, default_value_t = 0)]
    pub from: u32,

    #[arg(short, long, default_value_t = 0)]
    pub to: u32,

    #[arg(short, long, default_value_t = 0)]
    pub number: u32,

    #[arg(short, long, default_value_t = 9515)]
    pub port_chromedriver: u32,
}
