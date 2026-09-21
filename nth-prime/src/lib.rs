pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::<u32>::new();

    primes.push(2);
    let mut cnt = 3;
    while primes.len() as u32 <= n {
        if primes
            .iter()
            .take_while(|&p| p * p <= cnt)
            .any(|&p| cnt % p == 0)
        {
        } else {
            primes.push(cnt);
        }
        cnt += 2;
    }
    *primes.last().unwrap()
}
