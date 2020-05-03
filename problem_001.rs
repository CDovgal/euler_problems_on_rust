fn main() {
    let mut sum = 0;
    for n in 1..1000 {
        if n % 3 == 0 {
            sum += n;
            continue;
        }
        if n % 5 == 0 {
            sum += n;
        }
    }
    println! ("the answer is {}",  sum)
}
