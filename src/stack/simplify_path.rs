fn simplify_path(path: String) -> String {
    let mut stack = vec![];

    for s in path.split("/") {
        if s.is_empty() {
            continue
        }

        if s == ".." {
            stack.pop();
        } else if s != "." {
            stack.push(s);
        }

    }

    format!("/{}", stack.join("/"))
}