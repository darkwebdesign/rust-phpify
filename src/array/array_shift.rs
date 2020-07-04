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

/// Shift an element off the beginning of vector
///
/// # Description
///
/// array_shift() shifts the first value of the vector off and returns it, shortening the vector by
/// one element and moving everything down.
///
/// # Examples
///
/// Example #1 array_shift() example
///
/// ```
/// use phpify::array::array_shift;
///
/// let mut stack = vec!["orange", "banana", "apple", "raspberry"];
/// let fruit = array_shift(&mut stack).unwrap();
///
/// assert_eq!(stack, vec!["banana", "apple", "raspberry"]);
/// assert_eq!(fruit, "orange");
/// ```
pub fn array_shift<T>(array: &mut Vec<T>) -> Option<T> {
    if array.is_empty() {
        return None;
    }

    Some(array.remove(0))
}

#[cfg(test)]
mod tests {
    use crate::array::array_shift;

    #[test]
    fn test() {
        let mut vec = vec!["a", "b", "c"];
        assert_eq!(array_shift(&mut vec), Some("a"));
        assert_eq!(vec, vec!["b", "c"]);
        assert_eq!(array_shift::<usize>(&mut Vec::new()), None);
    }
}
