use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Backends {
    X11,
    Wayland,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct PassedArgs {
    #[arg(long = "backend")]
    backend: Option<Backends>,
    #[arg(
        long = "without-decorations",
        default_value_t = false,
        help = "Disable decorations"
    )]
    without_decorations: bool,
}

pub fn use_decorations() -> bool {
    !PassedArgs::parse().without_decorations
}

pub fn backend_to_use() -> Option<Backends> {
    PassedArgs::parse().backend
}
