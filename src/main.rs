use otp_exchange::otp::OneTimePad;

fn main() {
	let local_pad = OneTimePad::load_zip("bin/pad.zip", "bin/temp_pad");
	local_pad.decrypt_file("bin/sample.bin", "bin/decrypted_sample.txt");
} 
