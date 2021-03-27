/// An Array is an N-dimensional matrix with elements of type
/// T.
pub struct Array<T, const N: usize> {
    dims: [usize; N],
    data: Vec<T>,
}
