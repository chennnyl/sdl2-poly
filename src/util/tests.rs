#[cfg(test)]
pub mod math {
    use crate::util::math::*;

    #[test]
    fn diff() {
        assert_eq!(differentiate(vec![3.0, 1.0]), vec![3.0]); // first-degree
        assert_eq!(differentiate(vec![0.0]), vec![0.0]); // constant
        assert_eq!(differentiate(vec![2.0, 4.0, 3.0]), vec![4.0, 4.0]); // second-degree
        assert_eq!(n_differentiate(vec![5.0, 1.0, 40.0, 120.0, 5.0], 2), vec![60.0, 6.0, 80.0]); // n-derivative less than degree
        assert_eq!(n_differentiate(vec![5.0, 1.0, 40.0, 120.0, 5.0], 4), vec![120.0]); // n-derivative equal to degree
        assert_eq!(n_differentiate(vec![5.0, 1.0, 40.0, 120.0, 5.0], 6), vec![0.0]); // n-derivative higher than degree
    }

    #[test]
    fn polynomial() {
        let quad = poly_closure(vec![5.0, 20.0, 15.0]);
        assert_eq!(quad(10.0), 715.0);
    }
}