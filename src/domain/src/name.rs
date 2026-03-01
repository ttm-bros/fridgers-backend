/// バリデーション付き文字列型を生成するマクロ。
///
/// # 使い方
/// ```
/// define_name!(UserName, max = 25);              // min=1（空文字禁止）, max=25
/// define_name!(FridgeName, max = 50);            // min=1（空文字禁止）, max=50
/// define_name!(RawPassword, min = 10, max = 30); // min=10, max=30
/// ```
macro_rules! define_name {
    // max のみ: min=1 として委譲
    ($t:ident, max = $max:expr) => {
        $crate::name::define_name!($t, min = 1, max = $max);
    };
    // min と max 両方指定: 実装本体
    ($t:ident, min = $min:expr, max = $max:expr) => {
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
                Ok(Self {
                    value,
                    _hide_default_constructor: std::marker::PhantomData,
                })
            }
        }
    };
}

pub(crate) use define_name;
