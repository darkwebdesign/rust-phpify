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

/// Pop the element off the end of vector
///
/// # Description
///
/// array_pop() pops and returns the value of the last element of vector, shortening the vector by
/// one element.
///
/// # Examples
///
/// Example #1 array_pop() example
///
/// ```
/// use phpify::array::array_pop;
///
/// let mut stack = vec!["orange", "banana", "apple", "raspberry"];
/// let fruit = array_pop(&mut stack).unwrap();
///
/// assert_eq!(stack, vec!["orange", "banana", "apple"]);
/// assert_eq!(fruit, "raspberry");
/// ```
pub fn array_pop<T>(array: &mut Vec<T>) -> Option<T> {
    array.pop()
}

#[cfg(test)]
mod tests {
    use crate::array::array_pop;

    #[test]
    fn test() {
        let mut vec = vec!["a", "b", "c"];
        assert_eq!(array_pop(&mut vec), Some("c"));
        assert_eq!(vec, vec!["a", "b"]);
        assert_eq!(array_pop::<usize>(&mut Vec::new()), None);
    }
}
