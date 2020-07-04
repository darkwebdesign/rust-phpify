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

/// Searches the vector for a given value and returns the first corresponding index if successful
///
/// # Description
///
/// Searches for needle in haystack.
///
/// # Examples
///
/// Example #1 array_search() example
///
/// ```
/// use phpify::array::array_search;
///
/// let array = vec!["blue", "red", "green", "red"];
/// assert_eq!(array_search(&"green", &array).unwrap(), 2);
/// assert_eq!(array_search(&"red", &array).unwrap(), 1);
/// ```
pub fn array_search<T>(needle: &T, haystack: &Vec<T>) -> Option<usize>
    where
        T: PartialEq {

    if haystack.is_empty() {
        return None;
    }

    haystack.iter().position(|x| x == needle)
}

#[cfg(test)]
mod tests {
    use crate::array::array_search;

    #[test]
    fn test() {
        assert_eq!(array_search(&"a", &vec!["a", "b", "c"]), Some(0));
        assert_eq!(array_search(&"c", &vec!["a", "b", "c"]), Some(2));
        assert_eq!(array_search(&"d", &vec!["a", "b", "c"]), None);
        assert_eq!(array_search(&"a", &Vec::new()), None);
    }
}
