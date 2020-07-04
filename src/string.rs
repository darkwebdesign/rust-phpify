// Copyright (c) 2020 DarkWeb Design
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

// https://www.php.net/manual/en/ref.strings.php

/// Split a string by a string.
///
/// # Description
///
/// Returns an vec of strings, each of which is a substring of string formed by splitting it on
/// boundaries formed by the string delimiter.
///
/// # Parameters
///
/// **limit**
///
/// If limit is set and positive, the returned vec will contain a maximum of limit elements with the
/// last element containing the rest of string.
///
/// If the limit parameter is negative, all components except the last -limit are returned.
///
/// If the limit parameter is zero, then this is treated as 1.
///
/// # Examples
///
/// Example #1 explode() examples
///
/// ```
/// use phpify::string::explode;
///
/// let pizza = "piece1 piece2 piece3 piece4 piece5 piece6";
/// let pieces = explode(" ", pizza, std::isize::MAX).unwrap();
///
/// assert_eq!(pieces, ["piece1", "piece2", "piece3", "piece4", "piece5", "piece6"]);
/// ```
///
/// ```
/// use phpify::string::explode;
///
/// let data = "foo:*:1023:1000::/home/foo:/bin/sh";
/// let elements = explode(":", data, std::isize::MAX).unwrap();
///
/// assert_eq!(elements[0], "foo".to_string());
/// assert_eq!(elements[1], "*".to_string());
/// ```
///
/// Example #2 explode() return examples
///
/// ```
/// use phpify::string::explode;
///
/// let input1 = "hello";
/// let input2 = "hello,there";
/// let input3 = ",";
///
/// assert_eq!(explode(",", input1, std::isize::MAX).unwrap(), ["hello"]);
/// assert_eq!(explode(",", input2, std::isize::MAX).unwrap(), ["hello", "there"]);
/// assert_eq!(explode(",", input3, std::isize::MAX).unwrap(), ["", ""]);
/// ```
///
/// Example #3 limit parameter examples
///
/// ```
/// use phpify::string::explode;
///
/// let str = "one|two|three|four";
///
/// assert_eq!(explode("|", str, 2).unwrap(), ["one", "two|three|four"]);
/// assert_eq!(explode("|", str, -1).unwrap(), ["one", "two", "three"]);
/// ```
pub fn explode<D, S>(delimiter: D, string: S, limit: isize) -> Option<Vec<String>>
    where
        D: AsRef<str>,
        S: AsRef<str> {

    let delimiter = delimiter.as_ref();
    let string = string.as_ref();

    if delimiter.is_empty() {
        return None;
    }

    if limit == 0 || limit == 1 {
        return Some(vec![string.to_string()]);
    }

    let vec: Vec<String> = string.split(delimiter).map(String::from).collect();
    let vec_length = vec.len() as isize;

    if limit > vec_length {
        return Some(vec);
    }

    if limit > 0 {
        let (left, right) = vec.split_at(limit as usize - 1);
        let mut vec = left.to_vec();
        vec.push(right.join(delimiter));

        return Some(vec);
    }

    if limit < 0 {
        if limit <= (0 - vec_length) {
            return Some(vec![]);
        }
        let (left, _right) = vec.split_at((vec_length + limit) as usize);
        let vec = left.to_vec();

        return Some(vec);
    }

    Some(vec)
}

/// Join vec elements with a string.
///
/// # Description
///
/// Join vec elements with a glue string.
///
/// # Examples
///
/// Example #1 implode() example
///
/// ```
/// use phpify::string::implode;
///
/// let vec = vec!["lastname".to_string(), "email".to_string(), "phone".to_string()];
/// let comma_separated = implode(",", &vec);
///
/// assert_eq!(comma_separated, "lastname,email,phone");
/// ```
pub fn implode<G>(glue: G, pieces: &Vec<String>) -> String
    where
        G: AsRef<str> {

    pieces.join(glue.as_ref())
}

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
/// If string is less than start characters long, None will be returned.
///
/// **length**
///
/// If length is given and is positive, the string returned will contain at most length characters
/// beginning from start (depending on the length of string).
///
/// If length is given and is negative, then that many characters will be omitted from the end of
/// string (after the start position has been calculated when a start is negative). If start denotes
/// the position of this truncation or beyond, FALSE will be returned.
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
    use crate::string::*;

    #[test]
    fn explode_test() {
        assert_eq!(explode("|", "one|two|three", 3), Some(vec!["one".to_string(), "two".to_string(), "three".to_string()]));
        assert_eq!(explode("|", "one|two|three", 1), Some(vec!["one|two|three".to_string()]));
        assert_eq!(explode("|", "one|two|three", 2), Some(vec!["one".to_string(), "two|three".to_string()]));
        assert_eq!(explode("|", "one|two|three", -1), Some(vec!["one".to_string(), "two".to_string()]));
        assert_eq!(explode("|", "one|two|three", -3), Some(vec![]));
        assert_eq!(explode(",", "one|two|three", 1), Some(vec!["one|two|three".to_string()]));
        assert_eq!(explode("", "one|two|three", 3), None);
    }

    #[test]
    fn implode_test() {
        assert_eq!(implode("|", &vec!["one".to_string(), "two".to_string(), "three".to_string()]), "one|two|three".to_string());
        assert_eq!(implode("", &vec!["one".to_string(), "two".to_string(), "three".to_string()]), "onetwothree".to_string());
        assert_eq!(implode("", &vec![]), "".to_string());
    }

    #[test]
    fn strlen_test() {
        assert_eq!(strlen("Hello World"), 11);
        assert_eq!(strlen(""), 0);
    }

    #[test]
    fn strpos_test() {
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

    #[test]
    fn stripos_test() {
        assert_eq!(stripos("HELLO WORLD", "world", 0), Some(6));
    }

    #[test]
    fn substr_test() {
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

    // #[test]
    fn bla() {
        let pizza = "piece1 piece2 piece3 piece4 piece5 piece6".to_string();
        let pieces = explode(&" ".to_string(), &pizza, std::isize::MAX).unwrap();
        println!("### {:?}", pieces); // ["piece1", "piece2", "piece3", "piece4", "piece5", "piece6"]
        println!("### {:?}", pizza);

        assert_eq!(1, 0);
    }
}
