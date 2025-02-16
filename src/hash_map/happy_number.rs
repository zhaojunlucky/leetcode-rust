fn is_happy(n: i32) -> bool {
    let mut no = n;
    let mut set = std::collections::HashSet::new();
    set.insert(n);
    loop {
        let mut nn = no;
        no = 0;
        while nn > 0 {
            let t = nn/10;
            no += (nn-t*10).pow(2);
            nn = t;
        }

        if set.contains(&no) {
            break;
        }
        set.insert(no);
    }
    no == 1
}