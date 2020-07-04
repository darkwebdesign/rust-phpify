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

#[cfg(test)]
mod tests {
    use crate::string::implode;

    #[test]
    fn test() {
        assert_eq!(implode("|", &vec!["one".to_string(), "two".to_string(), "three".to_string()]), "one|two|three".to_string());
        assert_eq!(implode("", &vec!["one".to_string(), "two".to_string(), "three".to_string()]), "onetwothree".to_string());
        assert_eq!(implode("", &vec![]), "".to_string());
    }
}
