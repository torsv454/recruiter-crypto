use std::env;
use std::io;

trait Transform {
    fn transform(&self, text: &str) -> String;
}

struct AsciiHexEncoder {}
struct AsciiHexDecoder {}
struct AsciiBinaryEncoder {}
struct AsciiBinaryDecoder {}

fn decode_ascii_nary(text: &str, n: u32) -> String {
    text.split_whitespace()
        .map(|x| {
            std::char::from_u32(
                u32::from_str_radix(x, n).expect("Ooops, input is not nary encoded ascii."),
            ).expect("Invalid character?")
        }).collect()
}

impl Transform for AsciiHexDecoder {
    fn transform(&self, text: &str) -> String {
        decode_ascii_nary(text, 16)
    }
}

impl Transform for AsciiHexEncoder {
    fn transform(&self, text: &str) -> String {
        let mut result = String::new();
        text.chars()
            .for_each(|c| result.push_str(&format!("{:02x} ", c as usize)));
        result.pop();
        result
    }
}

impl Transform for AsciiBinaryDecoder {
    fn transform(&self, text: &str) -> String {
        decode_ascii_nary(text, 2)
    }
}

impl Transform for AsciiBinaryEncoder {
    fn transform(&self, text: &str) -> String {
        let mut result = String::new();
        text.chars()
            .for_each(|c| result.push_str(&format!("{:08b} ", c as usize)));
        result.pop();
        result
    }
}

#[derive(Debug)]
enum CreateTransformErr {
    NoSuchTransform,
}

fn create_transform(direction: &str, kind: &str) -> Result<Box<Transform>, CreateTransformErr> {
    println!("{}, {}", direction, kind);
    match (direction, kind) {
        ("decode", "asciihex") => Ok(Box::new(AsciiHexDecoder {})),
        ("encode", "asciihex") => Ok(Box::new(AsciiHexEncoder {})),
        ("encode", "asciibin") => Ok(Box::new(AsciiBinaryEncoder {})),
        ("decode", "asciibin") => Ok(Box::new(AsciiBinaryDecoder {})),
        _ => Err(CreateTransformErr::NoSuchTransform),
    }
}

fn print_usage() {
    println!(
        r#"Usage: recruiter-crypto [direction] [type]

Where
    direction is either encode or decode
    type is either asciihex or asciibin

"#
    );
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        print_usage();
        std::process::exit(1);
    }

    let coder = create_transform(&args[1], &args[2]);
    if let Ok(coder) = coder {
        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(0) => break,
                Ok(n) => println!("{}", coder.transform(&input)),
                Err(error) => {
                    println!("error: {}", error);
                    break;
                }
            }
        }
    } else {
        print_usage();
        std::process::exit(1);
    }
}

#[test]
fn can_decode_asciihex() {
    assert_eq!(
        "Rust is awesome!",
        create_transform("decode", "asciihex")
            .unwrap()
            .transform("52 75 73 74 20 69 73 20 61 77 65 73 6f 6d 65 21")
    );
}

#[test]
fn can_encode_asciihex() {
    assert_eq!(
        "52 75 73 74 20 69 73 20 61 77 65 73 6f 6d 65 21",
        create_transform("encode", "asciihex")
            .unwrap()
            .transform("Rust is awesome!")
    );
}

#[test]
fn can_encode_asciibin() {
    assert_eq!(
        "01010010 01110101 01110011 01110100 00100000 01101001 01110011 00100000 01100001 01110111 01100101 01110011 01101111 01101101 01100101 00100001",
        create_transform("encode", "asciibin").unwrap()
            .transform("Rust is awesome!")
    );
}

#[test]
fn can_decode_asciibin() {
    assert_eq!(
        "Rust is awesome!",
        create_transform("decode", "asciibin").unwrap()
            .transform("01010010 01110101 01110011 01110100 00100000 01101001 01110011 00100000 01100001 01110111 01100101 01110011 01101111 01101101 01100101 00100001")
    );
}
