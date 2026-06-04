//! List / collection utilities.

/// Chunk an iterator into fixed-size `Vec` slices.
pub fn chunks<T: Clone>(items: &[T], size: usize) -> Vec<Vec<T>> {
    items.chunks(size).map(<[T]>::to_vec).collect()
}
