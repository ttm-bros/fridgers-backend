/// 文字列の名前型を生成するマクロ。
/// 共通バリデーション（空文字不許可、最大長）を持つ値オブジェクトを定義する。
///
/// # 使い方
/// ```
/// define_name!(UserName);              // 空文字のみ禁止
/// define_name!(FridgeName, max = 50);  // 空文字禁止 + 最大50文字
/// ```
macro_rules! define_name {
    ($t:ident) => {
        $crate::name::define_name!($t, max = usize::MAX);
    };
    ($t:ident, max = $max:expr) => {
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

            fn try_from(name: String) -> $crate::error::Result<Self> {
                if name.is_empty() {
                    return Err($crate::error::Error::InvalidLengthRange(
                        concat!(stringify!($t), " cannot be empty").to_string(),
                    ));
                }
                if name.len() > $max {
                    return Err($crate::error::Error::InvalidLengthRange(
                        format!(
                            concat!(stringify!($t), " cannot exceed {} characters"),
                            $max,
                        ),
                    ));
                }
                Ok(Self {
                    value: name,
                    _hide_default_constructor: std::marker::PhantomData,
                })
            }
        }
    };
}

pub(crate) use define_name;
