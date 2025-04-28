use rand::seq::SliceRandom;

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnpqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        password.push(
            UPPERCASE
                .choose(&mut rng)
                .expect("UPPERCASE won't be empty")
                .to_owned(),
        );
    }
    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        password.push(
            LOWERCASE
                .choose(&mut rng)
                .expect("LOWERCASE won't be empty")
                .to_owned(),
        );
    }
    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(
            NUMBERS
                .choose(&mut rng)
                .expect("NUMBERS won't be empty")
                .to_owned(),
        );
    }
    if symbols {
        chars.extend_from_slice(SYMBOLS);
        password.push(
            SYMBOLS
                .choose(&mut rng)
                .expect("SYMBOLS won't be empty")
                .to_owned(),
        );
    }

    password.extend((0..length - password.len() as u8).map(|_| {
        chars
            .choose(&mut rng)
            .expect("Char won't be empty in this context")
            .to_owned()
    }));

    password.shuffle(&mut rng);

    let password = String::from_utf8(password).expect("Password contains valid UTF-8");

    // output password strength on stderr
    eprintln!(
        "Password strength: {}",
        zxcvbn::zxcvbn(&password, &[]).score()
    );

    Ok(password)
}
