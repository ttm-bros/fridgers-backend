use crate::string::define_string;

define_string!(UserName, max = 25);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn ascii_within_max_is_ok() {
        // 25文字のASCII（境界値）
        let name = "a".repeat(25);
        assert!(UserName::try_from(name).is_ok());
    }

    #[test]
    fn ascii_over_max_is_err() {
        let name = "a".repeat(26);
        assert!(matches!(
            UserName::try_from(name),
            Err(Error::InvalidLengthRange(_))
        ));
    }

    #[test]
    fn multibyte_counted_by_chars_not_bytes() {
        // 11文字 = UTF-8 で 33 バイト。25バイト判定だと弾かれてしまうがOKであるべき
        let name = "ケースインセンシティブ".to_string();
        assert_eq!(name.chars().count(), 11);
        assert_eq!(name.len(), 33);
        assert!(UserName::try_from(name).is_ok());
    }

    #[test]
    fn multibyte_at_max_boundary_is_ok() {
        // 25文字（マルチバイト）= 75バイト
        let name = "あ".repeat(25);
        assert_eq!(name.chars().count(), 25);
        assert!(UserName::try_from(name).is_ok());
    }

    #[test]
    fn multibyte_over_max_is_err() {
        let name = "あ".repeat(26);
        assert!(matches!(
            UserName::try_from(name),
            Err(Error::InvalidLengthRange(_))
        ));
    }

    #[test]
    fn empty_is_err() {
        // min=1（define_string! の `max = ...` バリエーションは min=1 にデフォルト）
        assert!(matches!(
            UserName::try_from(String::new()),
            Err(Error::InvalidLengthRange(_))
        ));
    }
}
