#[macro_export]
macro_rules! range {
    ($x: expr, $y: expr, $t: ident) => {{
        let ret: Box<dyn Iterator<Item = $t>>;
        if $y < $x {
            ret = Box::new(($y..=$x).rev())
        } else {
            ret = Box::new($x..=$y)
        }

        ret
    }};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn positive_range() {
        let mut range = range!(1, 3, u32);

        assert_eq!(range.next(), Some(1));
        assert_eq!(range.next(), Some(2));
        assert_eq!(range.next(), Some(3));
        assert_eq!(range.next(), None);
    }

    #[test]
    fn negative_range() {
        let mut range = range!(3, 1, u32);

        assert_eq!(range.next(), Some(3));
        assert_eq!(range.next(), Some(2));
        assert_eq!(range.next(), Some(1));
        assert_eq!(range.next(), None);
    }
}
