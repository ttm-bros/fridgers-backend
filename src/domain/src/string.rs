/// デフォルトの文字バリデーター。
/// Unicode英数字・スペース・ハイフン・アンダースコアを許可する。
/// 絵文字・特殊記号はデフォルトで禁止。
pub(crate) fn default_char_allowed(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, ' ' | '-' | '_')
}

/// バリデーション付き文字列型を生成するマクロ。
///
/// # 使い方
/// ```ignore
/// define_string!(UserName, max = 25);
/// // → min=1, max=25, デフォルトバリデーター（英数字・スペース・ハイフン・アンダースコアのみ）
///
/// define_string!(RawPassword, min = 10, max = 30);
/// // → min=10, max=30, デフォルトバリデーター
///
/// define_string!(Tag, max = 20, validator = |c: char| c.is_ascii_alphanumeric());
/// // → min=1, max=20, カスタムバリデーター
///
/// define_string!(Note, min = 0, max = 200, validator = |c: char| !c.is_control());
/// // → min=0, max=200, カスタムバリデーター
/// ```
macro_rules! define_string {
    // max のみ: デフォルトバリデーターで委譲
    ($t:ident, max = $max:expr) => {
        $crate::string::define_string!(
            $t,
            min = 1,
            max = $max,
            validator = $crate::string::default_char_allowed
        );
    };
    // min + max: デフォルトバリデーターで委譲
    ($t:ident, min = $min:expr, max = $max:expr) => {
        $crate::string::define_string!(
            $t,
            min = $min,
            max = $max,
            validator = $crate::string::default_char_allowed
        );
    };
    // max + カスタムバリデーター: min=1 で委譲
    ($t:ident, max = $max:expr, validator = $validator:expr) => {
        $crate::string::define_string!($t, min = 1, max = $max, validator = $validator);
    };
    // 全パラメーター指定: 実装本体
    ($t:ident, min = $min:expr, max = $max:expr, validator = $validator:expr) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $t {
            value: String,
            _hide_default_constructor: std::marker::PhantomData<()>,
        }

        impl $t {
            pub fn value(&self) -> &str {
                &self.value
            }
        }

        impl TryFrom<String> for $t {
            type Error = $crate::error::Error;

            fn try_from(value: String) -> $crate::error::Result<Self> {
                if value.len() < $min {
                    return Err($crate::error::Error::InvalidLengthRange(format!(
                        concat!(stringify!($t), " must be at least {} characters"),
                        $min,
                    )));
                }
                if value.len() > $max {
                    return Err($crate::error::Error::InvalidLengthRange(format!(
                        concat!(stringify!($t), " cannot exceed {} characters"),
                        $max,
                    )));
                }
                if let Some(c) = value.chars().find(|&c| !$validator(c)) {
                    return Err($crate::error::Error::InvalidFormat(format!(
                        concat!(stringify!($t), " contains an invalid character: '{}'"),
                        c,
                    )));
                }
                Ok(Self {
                    value,
                    _hide_default_constructor: std::marker::PhantomData,
                })
            }
        }
    };
}

pub(crate) use define_string;
