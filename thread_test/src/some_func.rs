pub fn some_func(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return some_func(n - 2) + some_func(n - 1);
}
