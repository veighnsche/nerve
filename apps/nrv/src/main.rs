#![forbid(unsafe_code)]

fn print_usage() {
    eprintln!(
        "nrv {}\n\nUsage:\n  nrv --version\n  nrv sync-capabilities\n",
        env!("CARGO_PKG_VERSION")
    );
}

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("--version") | Some("-V") => {
            println!("nrv {}", env!("CARGO_PKG_VERSION"));
        }
        Some("sync-capabilities") => {
            println!("sync-capabilities: TODO — generate capability snapshots (ADR-010)");
        }
        Some("--help") | Some("-h") | None => {
            print_usage();
        }
        Some(other) => {
            eprintln!("Unknown command: {}\n", other);
            print_usage();
            std::process::exit(2);
        }
    }
}
