use proc_state_testing::{routing, routing_a};
use proc_state_testing_macro::count;
use proc_state_testing_macro2::count2;

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
    let i = routing();
    assert_eq!(i, routing());
    let j = routing_a();
    assert_eq!(j, routing_a());

    let k = count2!();
}
