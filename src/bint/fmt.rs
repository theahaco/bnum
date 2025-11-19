#[cfg(any(feature = "alloc", test))]
use core::fmt::{Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex};
#[cfg(any(feature = "alloc", test))]
use crate::alloc::format;

macro_rules! fmt_trait {
    ($BInt: ident, $trait: tt) => {
        #[cfg(any(feature = "alloc", test))]
        impl<const N: usize> $trait for $BInt<N> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
                $trait::fmt(&self.bits, f)
            }
        }
    };
}
macro_rules! fmt {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        fmt_trait!($BInt, Binary);
        #[cfg(any(feature = "alloc", test))]
        impl<const N: usize> Display for $BInt<N> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
                f.pad_integral(!self.is_negative(), "", &format!("{}", self.unsigned_abs()))
            }
        }

        impl<const N: usize> core::fmt::Debug for $BInt<N> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                #[cfg(not(all(feature = "alloc", test)))]
                todo!();
                #[cfg(any(feature = "alloc", test))]
                Display::fmt(&self, f)
            }
        }
        #[cfg(any(feature = "alloc", test))]
        impl<const N: usize> LowerExp for $BInt<N> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
                let uint = self.unsigned_abs();
                f.pad_integral(!self.is_negative(), "", &format!("{:e}", uint))
            }
        }
        fmt_trait!($BInt, LowerHex);
        fmt_trait!($BInt, Octal);
        #[cfg(any(feature = "alloc", test))]
        impl<const N: usize> UpperExp for $BInt<N> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
                let uint = self.unsigned_abs();
                f.pad_integral(!self.is_negative(), "", &format!("{:E}", uint))
            }
        }
        
        fmt_trait!($BInt, UpperHex);
    };
}

#[cfg(test)]
crate::test::all_digit_tests! {
    crate::int::fmt::tests!(itest);
}

crate::macro_impl!(fmt);
