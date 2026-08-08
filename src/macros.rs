macro_rules! convertible_enum {
    (
        $(#[$attr:meta])*
        $name: ident,
        repr = $repr: ty,
        unknown = $unknown_variant: ident,
        {$($variant: ident = $value: expr),* $(,)?}
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
        $(#[$attr])*
        #[repr($repr)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl From<$repr> for $name {
            fn from(value: $repr) -> Self {
                match value {
                    $($value => $name::$variant,)*
                    _ => $name::$unknown_variant,
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
