use insert_sort::insert_sort::backward as bw;

#[test]
fn insert_sort_test() {
    let mut array = [30, 10, 60, 70, 50, 30, 20, 80, 40, 90];
    let want = [10, 20, 30, 30, 40, 50, 60, 70, 80, 90];

    bw::mono_increasing(&mut array);
    assert_eq!(array, want);
}
