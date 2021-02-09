use std::unimplemented;

use crate::{
    buffer::{Bitmap, NativeType},
    datatypes::{DataType, IntervalUnit},
};

use super::{primitive::PrimitiveArray, Array, BinaryArray, BooleanArray, NullArray, Utf8Array};

mod boolean;
mod null;
mod primitive;
mod utils;
mod variable_size;

impl PartialEq for &dyn Array {
    fn eq(&self, other: &Self) -> bool {
        equal(*self, *other)
    }
}

impl<T: NativeType> PartialEq<&dyn Array> for PrimitiveArray<T> {
    fn eq(&self, other: &&dyn Array) -> bool {
        equal(self, *other)
    }
}

impl<T: NativeType> PartialEq<PrimitiveArray<T>> for PrimitiveArray<T> {
    fn eq(&self, other: &Self) -> bool {
        equal(self, other)
    }
}

/// Compares the values of two [ArrayData] starting at `lhs_start` and `rhs_start` respectively
/// for `len` slots. The null buffers `lhs_nulls` and `rhs_nulls` inherit parent nullability.
///
/// If an array is a child of a struct or list, the array's nulls have to be merged with the parent.
/// This then affects the null count of the array, thus the merged nulls are passed separately
/// as `lhs_nulls` and `rhs_nulls` variables to functions.
/// The nulls are merged with a bitwise AND, and null counts are recomputed where necessary.
#[inline]
fn equal_values(
    lhs: &dyn Array,
    rhs: &dyn Array,
    lhs_nulls: &Option<Bitmap>,
    rhs_nulls: &Option<Bitmap>,
    lhs_start: usize,
    rhs_start: usize,
    len: usize,
) -> bool {
    match lhs.data_type() {
        DataType::Null => {
            let lhs = lhs.as_any().downcast_ref::<NullArray>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<NullArray>().unwrap();
            null::equal(lhs, rhs)
        }
        DataType::Boolean => {
            let lhs = lhs.as_any().downcast_ref::<BooleanArray>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<BooleanArray>().unwrap();
            boolean::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::UInt8 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<u8>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<u8>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::UInt16 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<u16>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<u16>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::UInt32 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<u32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<u32>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::UInt64 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<u64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<u64>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::Int8 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<i8>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<i8>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::Int16 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<i16>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<i16>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::Int32
        | DataType::Date32
        | DataType::Time32(_)
        | DataType::Interval(IntervalUnit::YearMonth) => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<i32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<i32>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::Int64
        | DataType::Date64
        | DataType::Interval(IntervalUnit::DayTime)
        | DataType::Time64(_)
        | DataType::Timestamp(_, _)
        | DataType::Duration(_) => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<i64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<i64>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::Float16 => unreachable!(),
        DataType::Float32 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<f32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<f32>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::Float64 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::Decimal(_, _) => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<i128>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<PrimitiveArray<i128>>().unwrap();
            primitive::equal(lhs, rhs, lhs_nulls, rhs_nulls, lhs_start, rhs_start, len)
        }
        DataType::Utf8 => {
            let lhs = lhs.as_any().downcast_ref::<Utf8Array<i32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<Utf8Array<i32>>().unwrap();
            variable_size::equal(
                lhs.offsets(),
                rhs.offsets(),
                lhs.values(),
                rhs.values(),
                lhs_nulls,
                rhs_nulls,
                lhs_start,
                rhs_start,
                len,
            )
        }
        DataType::LargeUtf8 => {
            let lhs = lhs.as_any().downcast_ref::<Utf8Array<i64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<Utf8Array<i64>>().unwrap();
            variable_size::equal(
                lhs.offsets(),
                rhs.offsets(),
                lhs.values(),
                rhs.values(),
                lhs_nulls,
                rhs_nulls,
                lhs_start,
                rhs_start,
                len,
            )
        }
        DataType::Binary => {
            let lhs = lhs.as_any().downcast_ref::<BinaryArray<i32>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<BinaryArray<i32>>().unwrap();
            variable_size::equal(
                lhs.offsets(),
                rhs.offsets(),
                lhs.values(),
                rhs.values(),
                lhs_nulls,
                rhs_nulls,
                lhs_start,
                rhs_start,
                len,
            )
        }
        DataType::LargeBinary => {
            let lhs = lhs.as_any().downcast_ref::<BinaryArray<i64>>().unwrap();
            let rhs = rhs.as_any().downcast_ref::<BinaryArray<i64>>().unwrap();
            variable_size::equal(
                lhs.offsets(),
                rhs.offsets(),
                lhs.values(),
                rhs.values(),
                lhs_nulls,
                rhs_nulls,
                lhs_start,
                rhs_start,
                len,
            )
        }
        _ => unimplemented!(),
        /*
        DataType::Null => {}
        DataType::FixedSizeBinary(_) => {}
        DataType::List(_) => {}
        DataType::FixedSizeList(_, _) => {}
        DataType::LargeList(_) => {}
        DataType::Struct(_) => {}
        DataType::Union(_) => {}
        DataType::Dictionary(_, _) => {}
        */
    }
}

/// Logically compares two [ArrayData].
/// Two arrays are logically equal if and only if:
/// * their data types are equal
/// * their lengths are equal
/// * their null counts are equal
/// * their null bitmaps are equal
/// * each of their items are equal
/// two items are equal when their in-memory representation is physically equal (i.e. same bit content).
/// The physical comparison depend on the data type.
/// # Panics
/// This function may panic whenever any of the [ArrayData] does not follow the Arrow specification.
/// (e.g. wrong number of buffers, buffer `len` does not correspond to the declared `len`)
pub fn equal(lhs: &dyn Array, rhs: &dyn Array) -> bool {
    let lhs_nulls = lhs.nulls();
    let rhs_nulls = rhs.nulls();
    utils::base_equal(lhs, rhs)
        && lhs.null_count() == rhs.null_count()
        && utils::equal_nulls(lhs_nulls, rhs_nulls, 0, 0, lhs.len())
        && equal_values(lhs, rhs, lhs_nulls, rhs_nulls, 0, 0, lhs.len())
}

#[cfg(test)]
mod tests {
    use crate::array::{BooleanArray, Offset, Primitive};

    use super::*;

    #[test]
    fn test_primitive() {
        let cases = vec![
            (
                vec![Some(1), Some(2), Some(3)],
                vec![Some(1), Some(2), Some(3)],
                true,
            ),
            (
                vec![Some(1), Some(2), Some(3)],
                vec![Some(1), Some(2), Some(4)],
                false,
            ),
            (
                vec![Some(1), Some(2), None],
                vec![Some(1), Some(2), None],
                true,
            ),
            (
                vec![Some(1), None, Some(3)],
                vec![Some(1), Some(2), None],
                false,
            ),
            (
                vec![Some(1), None, None],
                vec![Some(1), Some(2), None],
                false,
            ),
        ];

        for (lhs, rhs, expected) in cases {
            let lhs = Primitive::<i32>::from(&lhs).to(DataType::Int32);
            let rhs = Primitive::<i32>::from(&rhs).to(DataType::Int32);
            test_equal(&lhs, &rhs, expected);
        }
    }

    #[test]
    fn test_primitive_slice() {
        let cases = vec![
            (
                vec![Some(1), Some(2), Some(3)],
                (0, 1),
                vec![Some(1), Some(2), Some(3)],
                (0, 1),
                true,
            ),
            (
                vec![Some(1), Some(2), Some(3)],
                (1, 1),
                vec![Some(1), Some(2), Some(3)],
                (2, 1),
                false,
            ),
            (
                vec![Some(1), Some(2), None],
                (1, 1),
                vec![Some(1), None, Some(2)],
                (2, 1),
                true,
            ),
            (
                vec![None, Some(2), None],
                (1, 1),
                vec![None, None, Some(2)],
                (2, 1),
                true,
            ),
            (
                vec![Some(1), None, Some(2), None, Some(3)],
                (2, 2),
                vec![None, Some(2), None, Some(3)],
                (1, 2),
                true,
            ),
        ];

        for (lhs, slice_lhs, rhs, slice_rhs, expected) in cases {
            let lhs = Primitive::<i32>::from(&lhs).to(DataType::Int32);
            let lhs = lhs.slice(slice_lhs.0, slice_lhs.1);
            let rhs = Primitive::<i32>::from(&rhs).to(DataType::Int32);
            let rhs = rhs.slice(slice_rhs.0, slice_rhs.1);

            test_equal(&lhs, &rhs, expected);
        }
    }

    fn test_equal(lhs: &dyn Array, rhs: &dyn Array, expected: bool) {
        // equality is symmetric
        assert_eq!(equal(lhs, lhs), true, "\n{:?}\n{:?}", lhs, lhs);
        assert_eq!(equal(rhs, rhs), true, "\n{:?}\n{:?}", rhs, rhs);

        assert_eq!(equal(lhs, rhs), expected, "\n{:?}\n{:?}", lhs, rhs);
        assert_eq!(equal(rhs, lhs), expected, "\n{:?}\n{:?}", rhs, lhs);
    }

    #[test]
    fn test_boolean_equal() {
        let a = BooleanArray::from_slice([false, false, true]);
        let b = BooleanArray::from_slice([false, false, true]);
        test_equal(&a, &b, true);

        let b = BooleanArray::from_slice([false, false, false]);
        test_equal(&a, &b, false);
    }

    #[test]
    fn test_boolean_equal_null() {
        let a = BooleanArray::from(vec![Some(false), None, None, Some(true)]);
        let b = BooleanArray::from(vec![Some(false), None, None, Some(true)]);
        test_equal(&a, &b, true);

        let b = BooleanArray::from(vec![None, None, None, Some(true)]);
        test_equal(&a, &b, false);

        let b = BooleanArray::from(vec![Some(true), None, None, Some(true)]);
        test_equal(&a, &b, false);
    }

    #[test]
    fn test_boolean_equal_offset() {
        let a = BooleanArray::from_slice(vec![false, true, false, true, false, false, true]);
        let b = BooleanArray::from_slice(vec![true, false, false, false, true, false, true, true]);
        test_equal(&a, &b, false);

        let a_slice = a.slice(2, 3);
        let b_slice = b.slice(3, 3);
        test_equal(&a_slice, &b_slice, true);

        let a_slice = a.slice(3, 4);
        let b_slice = b.slice(4, 4);
        test_equal(&a_slice, &b_slice, false);

        // Elements fill in `u8`'s exactly.
        let mut vector = vec![false, false, true, true, true, true, true, true];
        let a = BooleanArray::from_slice(vector.clone());
        let b = BooleanArray::from_slice(vector.clone());
        test_equal(&a, &b, true);

        // Elements fill in `u8`s + suffix bits.
        vector.push(true);
        let a = BooleanArray::from_slice(vector.clone());
        let b = BooleanArray::from_slice(vector);
        test_equal(&a, &b, true);
    }

    fn binary_cases() -> Vec<(Vec<Option<String>>, Vec<Option<String>>, bool)> {
        let base = vec![
            Some("hello".to_owned()),
            None,
            None,
            Some("world".to_owned()),
            None,
            None,
        ];
        let not_base = vec![
            Some("hello".to_owned()),
            Some("foo".to_owned()),
            None,
            Some("world".to_owned()),
            None,
            None,
        ];
        vec![
            (
                vec![Some("hello".to_owned()), Some("world".to_owned())],
                vec![Some("hello".to_owned()), Some("world".to_owned())],
                true,
            ),
            (
                vec![Some("hello".to_owned()), Some("world".to_owned())],
                vec![Some("hello".to_owned()), Some("arrow".to_owned())],
                false,
            ),
            (base.clone(), base.clone(), true),
            (base, not_base, false),
        ]
    }

    fn test_generic_string_equal<O: Offset>() {
        let cases = binary_cases();

        for (lhs, rhs, expected) in cases {
            let lhs = lhs.iter().map(|x| x.as_deref()).collect::<Vec<_>>();
            let rhs = rhs.iter().map(|x| x.as_deref()).collect::<Vec<_>>();
            let lhs = Utf8Array::<O>::from(&lhs);
            let rhs = Utf8Array::<O>::from(&rhs);
            test_equal(&lhs, &rhs, expected);
        }
    }

    #[test]
    fn test_string_equal() {
        test_generic_string_equal::<i32>()
    }

    #[test]
    fn test_large_string_equal() {
        test_generic_string_equal::<i64>()
    }
}