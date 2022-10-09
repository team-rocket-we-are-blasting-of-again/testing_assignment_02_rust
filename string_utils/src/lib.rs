pub mod string_utils {

    pub fn reverse_string(string: &str) -> String {
        let mut reverse = String::new();
        for char in string.chars().rev() {
            reverse.push(char);
        }

        return reverse;
    }

    pub fn lowercase_string(string: &str) -> String {
        let mut lowercase = String::new();
        for char in string.chars() {
            if char.is_ascii_alphabetic() {
                let char_num = char as u8;
                if char_num > 90 {
                    lowercase.push(char);
                    break;
                }
                lowercase.push((char_num + 32) as char);
            }
        }
        return lowercase;
    }

    pub fn capitalize_string(string: &str) -> String {
        let mut capitalized = String::new();
        for char in string.chars() {
            if char.is_ascii_alphabetic() {
                let char_num = char as u8;
                if char_num < 97 {
                    capitalized.push(char);
                    break;
                }
                capitalized.push((char_num - 32) as char);
            }
        }
        return capitalized;
    }
}