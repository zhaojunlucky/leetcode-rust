fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    for t in &tokens {
        match t.as_str() {
            "+" => {
                let top1 = stack.pop().unwrap();
                let top2 = stack.pop().unwrap();
                stack.push(top1 + top2);
            },
            "-" => {
                let top1 = stack.pop().unwrap();
                let top2 = stack.pop().unwrap();
                stack.push(top2 - top1);
            },
            "*" => {
                let top1 = stack.pop().unwrap();
                let top2 = stack.pop().unwrap();
                stack.push(top1 * top2);
            },
            "/" => {
                let top1 = stack.pop().unwrap();
                let top2 = stack.pop().unwrap();
                stack.push(top2 / top1);
            },
            _ => {
                let num = t.parse::<i32>().unwrap();
                stack.push(num);
            }

        }
    }

    stack.pop().unwrap()

}