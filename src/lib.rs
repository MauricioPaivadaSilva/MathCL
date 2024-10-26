pub mod basic {

    pub mod numbers {
        pub const PI: f32 = 3.141592;
        pub const EULER: f32 = 2.718281;
    }

    pub fn sum(mut num: f32, n: u32) -> f32 {
        let mut i: u32 = 0;
        let start: f32 = num;
        while i < n {
            num += start;
            i += 1;
        }
        num
    }

    pub fn fat(mut num: u32) -> u32 {
        if num == 0 || num == 1 {
            num = 1;
        } else {
            let mut term: u32 = num - 1;
            while term >= 1 {
                num *= term;
                term -= 1;
            }
        }
        num
    }

    pub fn pow(mut num: f32, n: i32) -> f32 {
        if n < 0 {
            let start: f32 = num;
            let mut i: i32 = -1;
            num = 1.0/num;
            while i > n {
                num *= 1.0/start;
                i -= 1;
            }
        } else if n == 0 {
            num = 1.0;
        } else {
            let start: f32 = num;
            let mut i: i32 = 1;
            while i < n {
                num *= start;
                i += 1;
            }
        }
        num
    }

    pub fn module(mut num: f32) -> f32 {
        if num < 0.0 {
            num *= -1.0;
        }
        num
    }

    pub fn sqrt(num: f32, n: u32) -> f32 {
        let mut x: f32 = 1.0;
        let mut count: u8 = 0;
        while count < 100 {
            x = x - ((pow(x, n as i32) - num)/((n as f32)*(pow(x, (n as i32)-1))));
            count += 1;
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_1(){
        assert_eq!(basic::sum(1.0, 1), 2.0)
    }

    #[test]
    fn sum_2(){
        assert_eq!(basic::sum(1.0, 10), 11.0)
    }

    #[test]
    fn sum_3(){
        assert_eq!(basic::sum(3.0, 3), 12.0)
    }

    #[test]
    fn sum_4(){
        assert_eq!(basic::sum(-3.0, 3), -12.0)
    }

    #[test]
    fn fat_1() {
        assert_eq!(basic::fat(0), 1)
    }

    #[test]
    fn fat_2() {
        assert_eq!(basic::fat(1), 1)
    }

    #[test]
    fn fat_3() {
        assert_eq!(basic::fat(5), 120)
    }

    #[test]
    fn module_1() {
        assert_eq!(basic::module(2.0), 2.0)
    }

    #[test]
    fn module_2() {
        assert_eq!(basic::module(0.0), 0.0)
    }

    #[test]
    fn module_3() {
        assert_eq!(basic::module(-442.0), 442.0)
    }

    #[test]
    fn pow_1() {
        assert_eq!(basic::pow(-2.0, 2), 4.0)
    }

    #[test]
    fn pow_2() {
        assert_eq!(basic::pow(-2.0, 3), -8.0)
    }

    #[test]
    fn pow_3() {
        assert_eq!(basic::pow(19.0, 0), 1.0)
    }

    #[test]
    fn pow_4() {
        assert_eq!(basic::pow(2.0, -2), 0.25)
    }

    #[test]
    fn pow_5() {
        assert_eq!(basic::pow(-2.0, -3), -0.125)
    }

    #[test]
    fn sqrt_1() {
        assert_eq!(basic::sqrt(4.0, 2), 2.0)
    }

    #[test]
    fn sqrt_2() {
        assert_eq!(basic::sqrt(27.0, 3), 3.0)
    }

    #[test]
    fn sqrt_3() {
        assert_eq!(basic::sqrt(9.0, 2), 3.0)
    }
}
