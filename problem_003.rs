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
    let input = 600851475143_i128;
    let mut largestprime = 3;
    let mut maxsearch = input;
    let mut i = 3;
    while i <= maxsearch {
        if isprime(i) && maxsearch % i == 0{
            largestprime = i;
            maxsearch /= largestprime;
            println! ("largest prime so far is {}",  largestprime);
            println! ("current max search is {}",  maxsearch);
        }
        i += 2;
    }
    println! ("the answer is {}",  largestprime)
}
