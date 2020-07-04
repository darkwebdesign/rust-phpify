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

use crate::string::strpos;

/// Find the position of the first occurrence of a case-insensitive substring in a string.
///
/// # Description
///
/// Find the numeric position of the first occurrence of needle in the haystack string.
///
/// Unlike the strpos(), stripos() is case-insensitive.
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
/// Example #1 stripos() example
///
/// ```
/// use phpify::string::stripos;
///
/// let mystring = "ABC";
/// let findme = "a";
/// let pos = stripos(mystring, findme, 0).unwrap();
///
/// assert_eq!(pos, 0);
/// ```
pub fn stripos<H, N>(haystack: H, needle: N, offset: isize) -> Option<usize>
    where
        H: AsRef<str>,
        N: AsRef<str> {

    strpos(haystack.as_ref().to_lowercase(), needle.as_ref().to_lowercase(), offset)
}

#[cfg(test)]
mod tests {
    use crate::string::stripos;

    #[test]
    fn test() {
        assert_eq!(stripos("HELLO WORLD", "world", 0), Some(6));
    }
}
