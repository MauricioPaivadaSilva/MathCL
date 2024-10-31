pub mod basic {

    pub mod numbers {
        pub const PI: f32 = 3.141592;
        pub const EULER: f32 = 2.718281;
    }

    pub fn sum(mut num: f32, n: u32) -> f32 {
        let mut i: u32 = 1;
        let start: f32 = num;
        while i <= n {
            num += start;
            i += 1;
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

pub mod combinate {
    use crate::basic::pow;

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

    pub fn awr(num: u32, r: u32) -> u32 {
        let arg: f32;
        if r != 0 {
            arg = pow(num as f32, r as i32);
        } else {
            panic!("\n\tErr ::> argument r is null\n");
        }
        arg as u32
    }

    pub fn arrangement(num: u32, r: u32) -> u32 {
        let mut arg: u32;
        if r == 1 {
            arg = 1;
        } else if 1 <= r && r <= num {
            arg = num;
            let mut cont: u32 = 1;
            while cont < r && r <= num {
                arg *= num - cont;
                cont += 1;
            }
        } else {
            panic!("\n\tErr ::> argument r is null\n");
        }
        arg
    }

    pub fn permutation(num: u32) -> u32 {
        let arg: u32 = fat(num);
        arg
    }
}

#[cfg(test)]
mod basic_test {
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

#[cfg(test)]
mod combinate_test {
    use super::*;

    #[test]
    fn fat_1() {
        assert_eq!(combinate::fat(0), 1)
    }

    #[test]
    fn fat_2() {
        assert_eq!(combinate::fat(1), 1)
    }

    #[test]
    fn fat_3() {
        assert_eq!(combinate::fat(5), 120)
    }

    #[test]
    fn awr_1() {
        assert_eq!(combinate::awr(52, 3), 140_608)
    }

    #[test]
    fn awr_2() {
        assert_eq!(combinate::awr(2, 3), 8)
    }

    #[test]
    fn awr_3() {
        assert_eq!(combinate::awr(1, 1), 1)
    }

    #[test]
    fn arrangement_1() {
        assert_eq!(combinate::arrangement(52, 3), 132_600)
    }

    #[test]
    fn arrangement_2() {
        assert_eq!(combinate::arrangement(4, 3), 24)
    }

    #[test]
    fn arrangement_3() {
        assert_eq!(combinate::arrangement(37, 4), 1_585_080)
    }
}