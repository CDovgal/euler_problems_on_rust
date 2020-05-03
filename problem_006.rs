fn main() {
    let mut sumsquares = 0_i128;
    let mut squaresum = 0_i128;
    for i in 1..101 {
        sumsquares += i*i;
        squaresum += i;
    }
    squaresum *= squaresum;
    println! ("the answer is {}",  squaresum - sumsquares);
}
