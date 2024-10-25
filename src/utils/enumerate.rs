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
        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &$name) -> Option<std::cmp::Ordering> {
                let self_n: $ty = (*self).into();
                let other_n: $ty = (*other).into();
                self_n.partial_cmp(&other_n)
            }
        }
        impl Ord for $name {
            fn cmp(&self, other: &$name) -> std::cmp::Ordering {
                let self_n: $ty = (*self).into();
                let other_n: $ty = (*other).into();
                self_n.cmp(&other_n)
            }
        }
    };
}

pub(crate) use number_enum;
