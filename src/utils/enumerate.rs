macro_rules! number_enum {
    ($vis:vis $name:ident : $ty:ty { $($field:ident = $value:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        $vis enum $name {
            $($field = $value),+
        }
        impl Into<$ty> for $name {
            #[inline]
            fn into(self) -> $ty {
                self as $ty
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

pub(crate) use number_enum;
