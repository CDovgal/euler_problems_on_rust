fn isprime (n: i128) -> bool {
    let mut i = 2;
    while i <= n/2 {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {
    let target = 10001;
    let mut lastprime = 2;
    let mut primesfound = 1;
    let mut i = 3;
    while primesfound != target {
        if isprime(i) {
            lastprime = i;
            primesfound += 1;
            println! ("prime found {} with number {}", lastprime, primesfound);
        }
        i += 2;
    }
    println! ("the answer is {}",  lastprime);
}
