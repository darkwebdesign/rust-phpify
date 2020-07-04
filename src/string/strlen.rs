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

/// Get string length.
///
/// # Description
///
/// Returns the length of the given string.
///
/// # Examples
///
/// Example #1 strlen() example
///
/// ```
/// use phpify::string::strlen;
///
/// let str = "abcdef";
/// assert_eq!(strlen(str), 6);
///
/// let str = " ab cd ";
/// assert_eq!(strlen(str), 7);
/// ```
///
/// # notes
///
/// strlen() returns the number of bytes rather than the number of characters in a string.
pub fn strlen<S>(string: S) -> usize
    where
        S: AsRef<str>
{
    string.as_ref().len()
}

#[cfg(test)]
mod tests {
    use crate::string::strlen;

    #[test]
    fn test() {
        assert_eq!(strlen("Hello World"), 11);
        assert_eq!(strlen(""), 0);
    }
}
