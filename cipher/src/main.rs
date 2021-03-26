use clap::Clap;

/* Options definition*/
#[derive(Clap)]
#[clap(name = "cipher", about = "Command line application that cipher some message.", version = "0.1", author = "Théo P.")]
struct Opts {
    /// Vigenère Cipher, need a <String> key
    #[clap(short, long)]
    vigenere: bool,
		/// Caesar Cipher, need a <u8> key
    #[clap(short, long)]
    caesar: bool,
		/// Decipher <message> with the cipher use and the <key>
    #[clap(short, long)]
    decipher: bool,
		/// Encryption Key
    key: String,
		/// Message to cipher
    message: String,
}

fn main() {
	let opts: Opts = Opts::parse();

	if opts.vigenere && opts.caesar {
		eprintln!("error: several cipher error")
	}

	if opts.vigenere {
		let message = vigenere_cipher(opts.message, opts.key, opts.decipher);
		println!("{}", message);
	}else if opts.caesar {
		let key : u8 = match u8::from_str_radix(&opts.key, 10) {
				Ok(val) => val,
				Err(err) => std::process::exit({
            eprintln!("error: {:?}", err);
            0
        }),
		};

		let message = caesar_cipher(opts.message, key, opts.decipher);
		println!("{}", message);
	}else {
		eprintln!("error: cipher type error");
	}
	
}

fn vigenere_cipher(message : String, key : String, decipher : bool) -> String{
	let message = String::new();
	return message;
}

fn caesar_cipher(message : String, key : u8, decipher : bool) -> String{
	let message = String::new();
	return message;
}