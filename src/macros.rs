macro_rules! convertible_enum {
    (
        repr = $repr:ident,
        unknown = $unknown_variant:ident,
        $(#[$outer:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$inner:meta])*
                $variant:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        $(#[$outer])*
        #[repr($repr)]
        $vis enum $name {
            $(
                $(#[$inner])*
                $variant = $value,
            )*
        }

        impl From<$repr> for $name {
            fn from(value: $repr) -> Self {
                match value {
                    $(
                        $value => Self::$variant,
                    )*
                    _ => Self::$unknown_variant,
                }
            }
        }

        impl From<$name> for $repr {
            fn from(value: $name) -> Self {
                value as $repr
            }
        }
    };
}

pub(crate) use convertible_enum;
