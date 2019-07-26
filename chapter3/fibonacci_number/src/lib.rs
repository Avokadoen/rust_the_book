#![feature(test)]
extern crate test;


pub mod fibo {

    /// Retrieves fibonacci numbers using unsigned int
    ///
    /// # Examples
    ///
    /// ```
    /// use fibonacci_number::fibo
    /// 
    /// let arg = 8;
    /// let answer = fibo::fibonacci_number(arg);
    ///
    /// assert_eq!(21, answer);
    /// ```
    pub fn fibonacci_number(n: u64) -> u64{
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

        let mut fibo_number: u64 = 1;
        let mut prev_fibo_number: u64 = 1;
        for _i in 2..n {
            fibo_number = fibo_number + prev_fibo_number;
            prev_fibo_number = fibo_number - prev_fibo_number;
        }

        return fibo_number;
    }

    pub fn fibonacci_number_float(n: u64) -> f64 {
        if n == 0 {
            return 0.0;
        } else if n == 1 {
            return 1.0;
        }

        let mut fibo_number: f64 = 1.0;
        let mut prev_fibo_number: f64 = 1.0;
        for _i in 2..n {
            fibo_number = fibo_number + prev_fibo_number;
            prev_fibo_number = fibo_number - prev_fibo_number;
        }

        return fibo_number;
    }

    pub fn fibonacci_number_formula(n: f64) -> f64 {
        let big_phi: f64 = (1.0+5_f64.sqrt())/2.0;
        let small_phi: f64 = -1.0/big_phi;
        (big_phi.powf(n) - (small_phi).powf(n))/5_f64.sqrt()
    }

    // source : https://benjaminbrandt.com/fibonacci-in-rust/ 
        // modified to start at 0
    pub fn fibonacci_number_recursive(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci_number_recursive(n - 1) + fibonacci_number_recursive(n - 2),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{ Bencher, black_box};

    #[test]
    fn fibo_5_non_recursive() {
        assert_eq!(5, fibo::fibonacci_number(5));
    }

    #[test]
    fn fibo_10_non_recursive() {
        assert_eq!(55, fibo::fibonacci_number(10));
    }

    #[test]
    fn fibo_5_recursive() {
        assert_eq!(5, fibo::fibonacci_number_recursive(5));
    }

    #[test]
    fn fibo_10_recursive() {
        assert_eq!(55, fibo::fibonacci_number_recursive(10));
    }


    #[test]
    fn fibo_47_non_recursive() {
        assert_eq!(2971215073, fibo::fibonacci_number(47));
    }

    #[test]
    fn fibo_47_recursive() {
        assert_eq!(2971215073, fibo::fibonacci_number_recursive(47));
    }

    #[bench]
    fn fibo_93_non_recursive_bench(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(fibo::fibonacci_number(93));
            return n
        });
    }

}
