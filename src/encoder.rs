use std::{char, collections::HashMap, sync::LazyLock};

static CHARACTER_SET: LazyLock<HashMap<u8, char>> = LazyLock::new(|| {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
        .chars()
        .enumerate()
        .map(|(idx, char)| (idx as u8, char))
        .collect()
});

static CHARACTER_SET_INV: LazyLock<HashMap<char, u8>> =
    LazyLock::new(|| CHARACTER_SET.iter().map(|(idx, v)| (*v, *idx)).collect());

pub fn encode(mut value: i64) -> String {
    if value == 0 {
        return CHARACTER_SET[&0].to_string();
    }

    let mut s = String::new();

    while value != 0 {
        let head = value & 63;
        value >>= 6;
        s.push(CHARACTER_SET[&(head as u8)]);
    }

    s
}

pub fn decode(value: &str) -> eyre::Result<i64> {
    let characters = value
        .chars()
        .filter_map(|char| CHARACTER_SET_INV.get(&char))
        .collect::<Vec<_>>();

    if characters.len() != value.len() {
        eyre::bail!("The provided encoded value contained invalid characters")
    }

    Ok(characters.into_iter().fold(0, |mut v, byte| {
        v <<= 6;
        v += *byte as i64;
        v
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! validate_encode {
    ( $( $name:ident ( $in:expr ) -> $out:expr ),+ $(,)? ) => {
        $(
            #[test]
            fn $name() {
                let output = encode($in);
                assert_eq!(output, $out);

                let output = decode($out).expect("Failed to decode value");
                assert_eq!(output, $in);
            }
        )+
    };
}

    validate_encode! {
        encode_0(0) -> "A",
        encode_1(1) -> "B",
        encode_2(2) -> "C",
        encode_63(63) -> "_",
        encode_64(64) -> "AB",
        encode_128(128) -> "AC",
        encode_192(192) -> "AD",
        encode_max(i64::MAX) -> "__________H",
    }
}
