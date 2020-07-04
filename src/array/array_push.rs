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

/// Push one or more elements onto the end of vector
///
/// # Description
///
/// array_push() treats vector as a stack, and pushes the passed variable onto the end of vector.
/// The length of vector increases by one.
///
/// # Examples
///
/// Example #1 array_push() example
///
/// ```
/// use phpify::array::array_push;
///
/// let mut stack = vec!["orange", "banana"];
/// array_push(&mut stack, "apple");
///
/// assert_eq!(stack, vec!["orange", "banana", "apple"]);
/// ```
pub fn array_push<T>(array: &mut Vec<T>, value: T) {
    array.push(value)
}

#[cfg(test)]
mod tests {
    use crate::array::array_push;

    #[test]
    fn test() {
        let mut vec = vec!["a", "b"];
        array_push(&mut vec, "c");
        assert_eq!(vec, vec!["a", "b", "c"]);
    }
}
