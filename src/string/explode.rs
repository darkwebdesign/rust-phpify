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

#[cfg(test)]
mod tests {
    use crate::string::explode;

    #[test]
    fn test() {
        assert_eq!(explode("|", "one|two|three", 3), Some(vec!["one".to_string(), "two".to_string(), "three".to_string()]));
        assert_eq!(explode("|", "one|two|three", 1), Some(vec!["one|two|three".to_string()]));
        assert_eq!(explode("|", "one|two|three", 2), Some(vec!["one".to_string(), "two|three".to_string()]));
        assert_eq!(explode("|", "one|two|three", -1), Some(vec!["one".to_string(), "two".to_string()]));
        assert_eq!(explode("|", "one|two|three", -3), Some(vec![]));
        assert_eq!(explode(",", "one|two|three", 1), Some(vec!["one|two|three".to_string()]));
        assert_eq!(explode("", "one|two|three", 3), None);
    }
}
