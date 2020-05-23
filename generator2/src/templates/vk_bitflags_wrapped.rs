#[macro_export]
macro_rules! vk_bitflags_wrapped {
    ($name: ident, $all: expr, $flag_type: ty) => {

        impl Default for $name{
            fn default() -> $name {
                $name(0)
            }
        }

        impl $name {
            #[inline]
            pub const fn empty() -> $name {
                $name(0)
            }

            #[inline]
            pub const fn all() -> $name {
                $name($all)
            }

            #[inline]
            pub const fn from_raw(x: $flag_type) -> Self { $name(x) }

            #[inline]
            pub const fn as_raw(self) -> $flag_type { self.0 }

            #[inline]
            pub fn is_empty(self) -> bool {
                self == $name::empty()
            }

            #[inline]
            pub fn is_all(self) -> bool {
                self & $name::all() == $name::all()
            }

            #[inline]
            pub fn intersects(self, other: $name) -> bool {
                self & other != $name::empty()
            }

            /// Returns whether `other` is a subset of `self`
            #[inline]
            pub fn contains(self, other: $name) -> bool {
                self & other == other
            }
        }

        impl ::std::ops::BitOr for $name {
            type Output = $name;

            #[inline]
            fn bitor(self, rhs: $name) -> $name {
                $name (self.0 | rhs.0 )
            }
        }

        impl ::std::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $name) {
                *self = *self | rhs
            }
        }

        impl ::std::ops::BitAnd for $name {
            type Output = $name;

            #[inline]
            fn bitand(self, rhs: $name) -> $name {
                $name (self.0 & rhs.0)
            }
        }

        impl ::std::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $name) {
                *self = *self & rhs
            }
        }

        impl ::std::ops::BitXor for $name {
            type Output = $name;

            #[inline]
            fn bitxor(self, rhs: $name) -> $name {
                $name (self.0 ^ rhs.0 )
            }
        }

        impl ::std::ops::BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: $name) {
                *self = *self ^ rhs
            }
        }

        impl ::std::ops::Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(self, rhs: $name) -> $name {
                self & !rhs
            }
        }

        impl ::std::ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $name) {
                *self = *self - rhs
            }
        }

        impl ::std::ops::Not for $name {
            type Output = $name;

            #[inline]
            fn not(self) -> $name {
                self ^ $name::all()
            }
        }
    }
}
