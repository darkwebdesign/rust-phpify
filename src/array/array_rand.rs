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

use rand::Rng;

/// Pick one random key out of a vector
///
/// # Description
///
/// Picks one entry out of a vector, and returns the index of the random entry.
///
/// @TODO: It uses a pseudo random number generator that is not suitable for cryptographic purposes.
///
/// # Examples
///
/// Example #1 array_rand() example
///
/// ```
/// use phpify::array::array_rand;
///
/// let input = vec!["Neo", "Morpheus", "Trinity", "Cypher", "Tank"];
/// let rand_key = array_rand(&input).unwrap();
///
/// assert_eq!(rand_key <= 4, true);
/// ```
pub fn array_rand<T>(array: &Vec<T>) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let length = array.len();

    if length == 1 {
        return Some(0);
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, length - 1);

    Some(index)
}

#[cfg(test)]
mod tests {
    use crate::array::array_rand;

    #[test]
    fn test() {
        let rand = array_rand(&vec!["a", "b", "c"]);
        assert_eq!(rand.is_some(), true);
        assert_eq!(rand.unwrap() <= 2, true);
        assert_eq!(array_rand(&vec!["a"]), Some(0));
        assert_eq!(array_rand::<usize>(&Vec::new()), None);
    }
}
