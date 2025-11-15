#[cfg(feature = "no_alloc")]
#[allow(unused)]
mod alloc {
    pub mod prelude {
        pub use super::string::ToString;
        pub use super::string::StringExt;
        pub use super::vec::VecExt;
        pub use core::iter::Extend;
    }
    pub use crate::format;
    pub mod string {

        use crate::format;

        pub type String = arrayvec::ArrayString<102>;

        pub trait ToString {
            fn to_string(&self) -> String;
        }
        pub trait StringExt {
            fn into_bytes(&self) -> super::vec::Vec<u8>;
            unsafe fn from_utf8_unchecked(bytes: impl AsRef<[u8]>) -> String {
                Self::from_utf8(bytes).unwrap_unchecked()
            }
            fn from_utf8(bytes: impl AsRef<[u8]>) -> Option<String>;
            fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I);
            fn insert(&mut self, i: usize, c: char);
        }

        impl<T: core::fmt::Display> ToString for T {
            fn to_string(&self) -> String {
                format!("{self}")
            }
        }

        impl StringExt for String {
            fn into_bytes(&self) -> super::vec::Vec<u8> {
                super::vec::Vec::try_from(self.as_bytes()).unwrap()
            }

            fn from_utf8(bytes: impl AsRef<[u8]>) -> Option<Self> {
                let s = core::str::from_utf8(bytes.as_ref()).ok()?;
                let mut array_string = String::new();
                array_string.push_str(s);
                Some(array_string)
            }

            fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
                for c in iter {
                    self.push(c);
                }
            }

            fn insert(&mut self, i: usize, c: char) {
                insert_char(self, i, c).unwrap();
            }
        }

        use arrayvec::{ArrayString, CapacityError};
        use core::ptr;

        fn insert_char<const CAP: usize>(
            string: &mut ArrayString<CAP>,
            char_index: usize,
            c: char,
        ) -> Result<(), CapacityError<char>> {
            let char_len = c.len_utf8();
            let byte_pos = string
                .char_indices()
                .nth(char_index)
                .map(|(pos, _)| pos)
                .unwrap_or(string.len());

            if string.len() + char_len > string.capacity() {
                return Err(CapacityError::new(c));
            }
            #[allow(unsafe_code)]
            unsafe {
                let len = string.len();
                let ptr = string.as_mut_ptr();

                // Shift bytes right by char_len from byte_pos to end
                ptr::copy(
                    ptr.add(byte_pos),
                    ptr.add(byte_pos + char_len),
                    len - byte_pos,
                );

                // Write the char's UTF-8 bytes into the gap
                let mut buffer = [0u8; 4];
                c.encode_utf8(&mut buffer);
                ptr::copy_nonoverlapping(buffer.as_ptr(), ptr.add(byte_pos), char_len);

                // Update length
                string.set_len(len + char_len);
            }
            Ok(())
        }
    }
    pub mod vec {
        pub const MAX_CAPACITY: usize = 100;
        pub type Vec<T> = arrayvec::ArrayVec<T, MAX_CAPACITY>;
        pub trait VecExt<T> {
            fn resize(&mut self, new_len: usize, value: T);
            fn extend_from_slice(&mut self, other: &[T]);

            fn with_capacity(capacity: usize) -> Self;

            fn from_slice(slice: &[T]) -> Self;
            
        }

        impl<T> VecExt<T> for Vec<T>
        where
            T: Clone + Copy
        {
            fn resize(&mut self, new_len: usize, value: T) {
                let current_len = self.len();
                if new_len > current_len {
                    for _ in current_len..new_len {
                        self.push(value.clone());
                    }
                } else {
                    self.truncate(new_len);
                    
                }
            }

            fn extend_from_slice(&mut self, other: &[T]) {
                self.try_extend_from_slice(other).unwrap();
            }

            fn with_capacity(capacity: usize) -> Self {
                assert!(capacity <= MAX_CAPACITY, "Requested capacity exceeds ArrayVec capacity");
                Vec::new()
            }
            
            fn from_slice(slice: &[T]) -> Self {
                let mut v = Vec::new();
                v.extend_from_slice(slice);
                v
            }
            


        }
    }
}

#[cfg(not(feature = "no_alloc"))]
extern crate alloc;

#[allow(unused)]
pub use alloc::*;
