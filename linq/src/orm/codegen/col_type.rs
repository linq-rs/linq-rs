use crate::Variant;

use super::{Column, Primary};

macro_rules! impl_num_into {
    ($from:ty,$to:ty) => {
        impl<const AUTOINC: bool> From<$from> for Primary<$to, AUTOINC> {
            fn from(v: $from) -> Primary<$to, AUTOINC> {
                Primary {
                    value: Some(v as $to),
                }
            }
        }

        impl<const AUTOINC: bool> From<$to> for Primary<$from, AUTOINC> {
            fn from(v: $to) -> Primary<$from, AUTOINC> {
                Primary {
                    value: Some(v as $from),
                }
            }
        }

        impl From<$from> for Column<$to> {
            fn from(v: $from) -> Column<$to> {
                Column {
                    value: Some(v as $to),
                }
            }
        }

        impl From<$to> for Column<$from> {
            fn from(v: $to) -> Column<$from> {
                Column {
                    value: Some(v as $from),
                }
            }
        }
    };
}

impl_num_into!(i8, i32);
impl_num_into!(i16, i32);
impl_num_into!(i64, i32);
impl_num_into!(usize, i32);
impl_num_into!(usize, i8);
impl_num_into!(usize, i16);
impl_num_into!(usize, i64);
impl_num_into!(i16, i64);
impl_num_into!(i8, i64);

impl<T, const AUTOINC: bool> From<T> for Primary<T, AUTOINC>
where
    T: TryFrom<Variant>,
{
    fn from(v: T) -> Primary<T, AUTOINC> {
        Primary { value: Some(v) }
    }
}

impl<T> From<T> for Column<T>
where
    T: TryFrom<Variant>,
{
    fn from(v: T) -> Column<T> {
        Column { value: Some(v) }
    }
}

impl<'a, const AUTOINC: bool> From<&'a str> for Primary<String, AUTOINC> {
    fn from(v: &'a str) -> Primary<String, AUTOINC> {
        Primary {
            value: Some(v.to_owned()),
        }
    }
}

impl<'a> From<&'a str> for Column<String> {
    fn from(v: &'a str) -> Column<String> {
        Column {
            value: Some(v.to_owned()),
        }
    }
}
