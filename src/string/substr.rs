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

/// Return part of a string.
///
/// # Description
///
/// Returns the portion of string specified by the start and length parameters.
///
/// # Parameters
///
/// **start**
///
/// If start is non-negative, the returned string will start at the start'th position in string,
/// counting from zero. For instance, in the string 'abcdef', the character at position 0 is 'a',
/// the character at position 2 is 'c', and so forth.
///
/// If start is negative, the returned string will start at the start'th character from the end of
/// string.
///
/// If string is less than start characters long, *None* will be returned.
///
/// **length**
///
/// If length is given and is positive, the string returned will contain at most length characters
/// beginning from start (depending on the length of string).
///
/// If length is given and is negative, then that many characters will be omitted from the end of
/// string (after the start position has been calculated when a start is negative). If start denotes
/// the position of this truncation or beyond, *NONE* will be returned.
///
/// If length is given and is 0, an empty string will be returned.
///
/// # Examples
///
/// Example #1 substr() examples
///
/// ```
/// use phpify::string::substr;
///
/// assert_eq!(substr("abcdef", 1, std::isize::MAX).unwrap(), "bcdef");
/// assert_eq!(substr("abcdef", 1, 3).unwrap(), "bcd");
/// assert_eq!(substr("abcdef", 0, 4).unwrap(), "abcd");
/// assert_eq!(substr("abcdef", 0, 8).unwrap(), "abcdef");
/// assert_eq!(substr("abcdef", -1, 1).unwrap(), "f");
/// ```
///
/// Example #2 using a negative start
///
/// ```
/// use phpify::string::substr;
///
/// assert_eq!(substr("abcdef", -1, std::isize::MAX).unwrap(), "f");
/// assert_eq!(substr("abcdef", -2, std::isize::MAX).unwrap(), "ef");
/// assert_eq!(substr("abcdef", -3, 1).unwrap(), "d");
/// ```
///
/// Example #3 using a negative length
///
/// ```
/// use phpify::string::substr;
///
/// assert_eq!(substr("abcdef", 0, -1).unwrap(), "abcde");
/// assert_eq!(substr("abcdef", 2, -1).unwrap(), "cde");
/// assert_eq!(substr("abcdef", 4, -4), None);
/// assert_eq!(substr("abcdef", -3, -1).unwrap(), "de");
/// ```
pub fn substr<S>(string: S, start: isize, length: isize) -> Option<String>
    where
        S: AsRef<str> {

    let string = string.as_ref();
    let mut start = start;
    let mut length = length;
    let string_length = string.len() as isize;

    if start > 0 && start >= string_length {
        return None;
    }

    if length == 0 {
        return None;
    }

    if start < 0 {
        start = string_length + start;
        if start < 0 {
            start = 0;
        }
    }

    if length < 0 {
        if start > string_length + length {
            return None;
        }
        length = string_length + length - start;
    }

    if start == 0 && length == string_length {
        return Some(string.to_string());
    }

    Some(string.chars().skip(start as usize).take(length as usize).collect::<String>())
}

#[cfg(test)]
mod tests {
    use crate::string::substr;

    #[test]
    fn test() {
        let string = &"Hello World".to_string();

        assert_eq!(substr(string, 0, 5), Some("Hello".to_string()));
        assert_eq!(substr(string, 6, 5), Some("World".to_string()));
        assert_eq!(substr(string, 0, 20), Some("Hello World".to_string()));
        assert_eq!(substr(string, 0, -6), Some("Hello".to_string()));
        assert_eq!(substr(string, 3, -3), Some("lo Wo".to_string()));
        assert_eq!(substr(string, -11, 11), Some("Hello World".to_string()));
        assert_eq!(substr(string, -20, 11), Some("Hello World".to_string()));
        assert_eq!(substr(string, 0, 0), None);
        assert_eq!(substr(string, 11, 1), None);
        assert_eq!(substr(string, 7, -5), None);
    }
}
