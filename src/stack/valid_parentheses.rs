fn is_valid(s: String) -> bool {
    if s.len() %2 != 0 {
        return false
    }
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else {
            if let Some(v) = stack.pop() {
                match v {
                    '(' => {
                        if c != ')' {
                            return false;
                        }
                    },
                    '[' => {
                        if c != ']' {
                            return false;
                        }
                    },
                    '{' => {
                        if c != '}' {
                            return false;
                        }
                    },

                    _ => {}
                }
            } else {
                return false;
            }
        }

    }

    stack.is_empty()
}