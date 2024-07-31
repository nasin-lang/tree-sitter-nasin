macro_rules! enumerate {
    ($vis:vis $name:ident : $ty:ty { $($field:ident = $value:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        $vis enum $name {
            $($field),+
        }
        impl Into<$ty> for $name {
            fn into(self) -> $ty {
                match self {
                    $(Self::$field => $value),+
                }
            }
        }
        impl TryFrom<$ty> for $name {
            type Error = ();
            fn try_from(value: $ty) -> Result<Self, ()> {
                match value {
                    $($value => Ok(Self::$field)),+,
                    _ => Err(()),
                }
            }
        }
    };
}

pub(crate) use enumerate;
