use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use qrcode_generator::QrCodeEcc;

fn main() {
    let mut rng = thread_rng();
    let chars: String =
        (0..24).map(|_| rng.sample(Alphanumeric) as char).collect();
    println!("Random chars: {}", chars);

    let result = qrcode_generator::to_png_to_file(
        &chars,
        QrCodeEcc::Low,
        1024,
        "./qr-code.png"
    ).unwrap();
}
