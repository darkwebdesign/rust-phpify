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

/// Prepend one element to the beginning of an vector
///
/// # Description
///
/// array_unshift() prepends passed element to the front of the vector.
///
/// # Examples
///
/// Example #1 array_shift() example
///
/// ```
/// use phpify::array::array_unshift;
///
/// let mut queue = vec!["orange", "banana"];
/// array_unshift(&mut queue, "apple");
///
/// assert_eq!(queue, vec!["apple", "orange", "banana"]);
/// ```
pub fn array_unshift<T>(array: &mut Vec<T>, value: T) {
    array.insert(0, value);
}

#[cfg(test)]
mod tests {
    use crate::array::array_unshift;

    #[test]
    fn test() {
        let mut vec = vec!["b", "c"];
        array_unshift(&mut vec, "a");
        assert_eq!(vec, vec!["a", "b", "c"]);
    }
}
