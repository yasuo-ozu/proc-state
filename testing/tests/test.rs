use proc_state_testing::count;

#[test]
fn test() {
    let v1 = count!(
        let v2 = count!();
        assert_eq!(v2, 2);
    );
    assert_eq!(v1, 1);
    let v3 = count!(
        let tup = ((count!(),), count!());
        assert_eq!(tup, ((4,), 5));
    );
    assert_eq!(v3, 3);
}
