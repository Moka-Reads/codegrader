
#[cfg(test)]
mod q01tests{
    use codegrader::q01::add;
    #[test]
    fn test_add_array() {
        let a_vals = [1, 2, 40, 34, 54];
        let b_vals = [43, 2, 1, 2, 4];
        let results = [44, 4, 41, 36, 58];

        for i in 0..5{
            let a = a_vals[i];
            let b = b_vals[i];
            let result = results[i];

            assert_eq!(add(a, b), result);
        }
    }
}
