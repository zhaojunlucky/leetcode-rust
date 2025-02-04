fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut lines:Vec<String> = vec![];

    let mut i = 0;
    let mut j;

    while i < words.len() {
        let mut size = words[i].len();
        j = i + 1;
        while j < words.len() {
            if size + words[j].len() + j - i > max_width as usize {
                break;
            }
            size += words[j].len();
            j += 1;
        }
        let mut line = String::new();
        let n_space = max_width as usize - size;

        if j == words.len() || i + 1 == j {
            for k in i..j {
                line.push_str(&*words[k]);
                if k != j - 1 {
                    line.push_str(" ");
                }
            }
            // insert all let space
            line.push_str(&" ".repeat(n_space - (j - i - 1)).as_str());

        } else {
            let n = n_space/(j - i - 1);
            let space = " ".repeat(n);
            let mut r = (n_space%(j - i - 1))  as i32;
            for k in i..j {
                line.push_str(&*words[k]);
                if k != j - 1 {
                    line.push_str(space.as_str());
                }
                if r > 0 {
                    line.push(' ');
                    r -= 1;
                }
            }
        }

        i = j;
        lines.push(line);
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(full_justify("Hello".split_ascii_whitespace().map(String::from).collect(), 7), vec!["Hello  "]);

        assert_eq!(full_justify("Hello".split_ascii_whitespace().map(String::from).collect(), 6), vec!["Hello "]);
        assert_eq!(full_justify("Hello".split_ascii_whitespace().map(String::from).collect(), 5), vec!["Hello"]);

        assert_eq!(full_justify("This is an example of text justification.".split_ascii_whitespace().map(String::from).collect(), 16), vec!["This    is    an", "example  of text", "justification.  "]);
        assert_eq!(full_justify("What must be acknowledgment shall be".split_ascii_whitespace().map(String::from).collect(), 16), vec!["What   must   be", "acknowledgment  ", "shall be        "]);
        assert_eq!(full_justify("Science is what we understand well enough to explain to a computer. Art is everything else we do".split_ascii_whitespace().map(String::from).collect(), 20), vec![ "Science  is  what we",
                                                                                                                                                                                  "understand      well",
                                                                                                                                                                                  "enough to explain to",
                                                                                                                                                                                  "a  computer.  Art is",
                                                                                                                                                                                  "everything  else  we",
                                                                                                                                                                                  "do                  "]);

    }
}