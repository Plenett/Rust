use clap::Clap;

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

	println!("input = {}", opts.message);
	println!("isVigenere = {}", opts.vigenere);
	println!("Key = {}", opts.key);
	
}

fn vigenere_cipher(message : String, key : String, decipher : bool) -> String{

}

fn caesar_cipher(message : String, key : u8, decipher : bool) -> String{

}