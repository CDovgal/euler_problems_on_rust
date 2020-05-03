fn ispalindrome (n: i32) -> bool {
    let ispalindrome: String = n.to_string();
    for i in 0..ispalindrome.len()/2 {
        if ispalindrome.as_bytes()[i] != ispalindrome.as_bytes()[ispalindrome.len()-i-1] {
            return false;
        }
    }

    return true;
}

fn devider (n: i32) -> i32 {
    let mut i = 999;
    while i > 99 {
        if n % i == 0  && n / i < 1000 {
            return i;
        }
        i -= 1;
    }
    return 0;
}

fn main() {
    let mut palindrome = 999999;
    while palindrome > 99999 {
        if ispalindrome (palindrome) {
            println! ("palindrome found: {}", palindrome);
            let dev = devider (palindrome);
            if dev != 0 {
                println! ("devider found! {}", dev);
                break;
            }
        }
        palindrome -= 1;
    }

    println! ("the answer is {}",  palindrome);
}
