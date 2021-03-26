use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Théo P.")]
struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
		/// Some input. Because this isn't an Option<T> it's required to be used
    input: i32,
}

fn main() {
	let opts: Opts = Opts::parse();

	println!("input = {}", opts.input);
}
