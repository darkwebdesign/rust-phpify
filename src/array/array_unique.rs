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

/// Removes duplicate values from a vector
///
/// # Description
///
/// Takes an input vector and returns a new vector without duplicate values.
///
/// # Examples
///
/// Example #1 array_unique() example
///
/// ```
/// use phpify::array::array_unique;
///
/// let input = vec!["green", "red", "green", "blue", "red"];
/// assert_eq!(array_unique(&input), [&"green", &"red", &"blue"]);
/// ```
pub fn array_unique<T>(array: &Vec<T>) -> Vec<&T>
    where
        T: PartialEq {

    let mut new_array = Vec::new();

    for value in array {
        if !new_array.contains(&value) {
            new_array.push(value);
        }
    }

    new_array
}

#[cfg(test)]
mod tests {
    use crate::array::array_unique;

    #[test]
    fn test() {
        assert_eq!(array_unique(&vec!["a", "b", "a", "c", "b"]), [&"a", &"b", &"c"]);
    }
}
