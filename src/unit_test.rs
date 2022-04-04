struct Rectancle {
    width: u32,
    height: u32,
}

impl Rectancle {
    fn compare_area(&self, other: &Rectancle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn double_value(a: i32) -> i32 {
    a * 2
}

fn greeting(name: &str) -> String {
    format!("Hello {} san", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_is_larger() {
        let a = Rectancle {
            width: 5,
            height: 5,
        };
        let b = Rectancle {
            width: 3,
            height: 3,
        };
        assert!(a.compare_area(&b));
    }

    #[test]
    fn test_a_is_smaller() {
        let a = Rectancle {
            width: 3,
            height: 3,
        };
        let b = Rectancle {
            width: 5,
            height: 5,
        };
        assert!(!(a.compare_area(&b)));
    }

    #[test]
    fn test_double() {
        assert_eq!(6, double_value(3));
    }
    #[test]
    fn test_contains_name() {
        let res = greeting("rust");
        assert!(res.contains("rust"));
    }
}
