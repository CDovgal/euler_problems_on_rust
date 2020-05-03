fn main() {
    let mut fib1 = 1;
    let mut fib2 = 2;
    let limit = 4000000;
    let mut sum = fib2;
    while fib1 < limit || fib2 < limit  {
        fib1 += fib2;
        fib2 += fib1;
        if fib1 % 2 == 0 {
            sum += fib1
        }
        if fib2 % 2 == 0 {
            sum += fib2
        }
    }
    println! ("fib1 is {0}, fib2 is {1}", fib1, fib2);
    println! ("the answer is {}",  sum);
}
