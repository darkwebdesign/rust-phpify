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

/// Make a string's first character uppercase
///
/// # Description
///
/// Returns a string with the first character of str capitalized, if that character is alphabetic.
///
/// Note that 'alphabetic' is determined by the current locale. For instance, in the default "C"
/// locale characters such as umlaut-a (ä) will not be converted.
///
/// # Examples
///
/// Example #1 lcfirst() example
///
/// ```
/// use phpify::string::ucfirst;
///
/// let foo = "hello world!";
/// assert_eq!(ucfirst(foo), "Hello world!");
/// ```
pub fn ucfirst<S>(str: S) -> String
    where
        S: AsRef<str> {

    let mut chars = str.as_ref().chars();

    match chars.next() {
        None => String::new(),
        Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use crate::string::ucfirst;

    #[test]
    fn test() {
        assert_eq!(ucfirst("abc"), "Abc");
        assert_eq!(ucfirst("äbc"), "Äbc");
        assert_eq!(ucfirst("δξπ"), "Δξπ");
    }
}
