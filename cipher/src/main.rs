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
	
	/* Invert key if decipher */
	let key = if decipher {key.chars().map(|c| ((b'z' - c as u8 + 1)%26 + b'a') as char ).collect()} else {key};


	(0..message.len()).map(|i| {
			let c = message.as_bytes()[i] as char;
			let key = key.as_bytes()[i%key.len()] as u8 - b'a';

			if c.is_ascii_alphabetic() {
					let first = if c.is_ascii_lowercase() {b'a'} else {b'A'};
					(first + (c as u8 + key - first) % 26) as char
			}	else {
				c
			}
	})
	.collect()
}

fn caesar_cipher(message : String, key : u8, decipher : bool) -> String{

	/* Invert key if decipher */
	let key = if decipher {26 - key} else {key};

	message.chars().map(|c| {
			if c.is_ascii_alphabetic() {
					let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
					(first + (c as u8 + key - first) % 26) as char
			} else {
					c
			}
	})
	.collect()
}