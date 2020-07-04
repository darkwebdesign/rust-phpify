// Copyright (c) 2020 DarkWeb Design
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Apply a user supplied function to every member of a vector
///
/// # Description
///
/// Applies the user-defined callback function to each element of the vector.
///
/// **callback**
///
/// Callback takes on two parameters. The vector parameter's value being the first, and the index
/// second.
///
/// Only the values of the vector may potentially be changed, i.e., the programmer cannot add, unset
/// or reorder elements.
///
/// # Examples
///
/// Example #1 array_walk() example
///
/// ```
/// use phpify::array::array_walk;
///
/// let mut fruits = vec![
///     "lemon".to_string(),
///     "orange".to_string(),
///     "banana".to_string(),
///     "apple".to_string(),
/// ];
///
/// fn test_alter(item: &mut String, index: usize) {
///     *item = format!("fruit: {}", *item);
/// }
///
/// array_walk(&mut fruits, test_alter);
///
/// assert_eq!(fruits[0], "fruit: lemon");
/// assert_eq!(fruits[1], "fruit: orange");
/// assert_eq!(fruits[2], "fruit: banana");
/// assert_eq!(fruits[3], "fruit: apple");
/// ```
pub fn array_walk<T>(array: &mut Vec<T>, callback: impl Fn(&mut T, usize) + 'static) {
    for (index, value) in array.iter_mut().enumerate() {
        callback(value, index);
    }
}

#[cfg(test)]
mod tests {
    use crate::array::array_walk;

    #[test]
    fn test() {
        let mut vec = vec![1, 2, 3];
        array_walk(&mut vec, |value, index| *value = *value * index);
        assert_eq!(vec, [0, 2, 6]);
    }
}
