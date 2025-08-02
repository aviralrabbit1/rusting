pub fn reverse_string(str: &str) -> String {

    str.chars().rev().collect()
    // input.chars(): converts the string slice into an iterator of its characters.
    //     .rev(): reverses the order of the characters in the iterator.
    //     .collect(): collects the characters from the reversed iterator into a new String.

    //     similar to javascript
    //     str.split("").reverse().join("")
}
