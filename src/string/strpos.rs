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

/// Find the position of the first occurrence of a substring in a string.
///
/// # Description
///
/// Find the numeric position of the first occurrence of needle in the haystack string.
///
/// # Parameters
///
/// **offset**
///
/// If the offset is negative, the search will start this number of characters counted from the end
/// of the string.
///
/// # Examples
///
/// Example #1 strpos() example
///
/// ```
/// use phpify::string::strpos;
///
/// let mystring = "abc";
/// let findme = "a";
/// let pos = strpos(mystring, findme, 0).unwrap();
///
/// assert_eq!(pos, 0);
/// ```
///
/// Example #2 using an offset
///
/// ```
/// use phpify::string::strpos;
///
/// let newstring = "abcdef abcdef";
/// let pos = strpos(newstring, "a", 1).unwrap();
///
/// assert_eq!(pos, 7);
/// ```
///
pub fn strpos<H, N>(haystack: H, needle: N, offset: isize) -> Option<usize>
    where
        H: AsRef<str>,
        N: AsRef<str> {

    let mut haystack = haystack.as_ref().to_string();
    let needle = needle.as_ref();
    let mut offset = offset;
    let haystack_length = haystack.len() as isize;

    if offset > 0 && offset > haystack_length {
        return None;
    }

    if offset < 0 {
        if offset < (0 - haystack_length) {
            return None;
        }
        offset = haystack_length + offset;
    }

    if offset == 0 {
        return haystack.find(needle);
    }

    haystack = haystack.chars().skip(offset as usize).collect();

    match haystack.find(needle) {
        None => None,
        Some(position) => Some(position + offset as usize),
    }
}

#[cfg(test)]
mod tests {
    use crate::string::strpos;

    #[test]
    fn test() {
        let haystack = "Hello World";
        let needle = "World";

        assert_eq!(strpos(haystack, needle, 0), Some(6));
        assert_eq!(strpos(haystack, needle, 6), Some(6));
        assert_eq!(strpos(haystack, needle, -5), Some(6));
        assert_eq!(strpos(haystack, needle, -11), Some(6));
        assert_eq!(strpos(haystack, needle, 7), None);
        assert_eq!(strpos(haystack, needle, 11), None);
        assert_eq!(strpos(haystack, needle, -12), None);
    }
}
