use crate::string::define_string;

define_string!(RawPassword, min = 10, max = 30);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn ten_chars_is_ok() {
        assert!(RawPassword::try_from("a".repeat(10)).is_ok());
    }

    #[test]
    fn nine_chars_is_err() {
        assert!(matches!(
            RawPassword::try_from("a".repeat(9)),
            Err(Error::InvalidLengthRange(_))
        ));
    }

    #[test]
    fn multibyte_min_counted_by_chars_not_bytes() {
        // 10文字のマルチバイトは min=10 を満たす（= 30 バイト）
        // バイト数判定だと「30 バイト >= 10 バイト」で偶然 OK になるが、
        // バイト判定だと逆に「3文字 = 9バイト < 10」で 3文字でも弾けてしまう。
        // 文字数判定なら「3文字 < 10」で確実に弾ける。
        assert!(RawPassword::try_from("あ".repeat(10)).is_ok());
        assert!(matches!(
            RawPassword::try_from("あ".repeat(9)),
            Err(Error::InvalidLengthRange(_))
        ));
    }
}
