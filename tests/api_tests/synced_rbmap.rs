use persi_ds::sync::RBMap;

#[test]
fn store_str_as_key() {
    let rbmap: RBMap<&'_ str, i32> = RBMap::new();

    let key = "a";
    let rbmap = rbmap.inserted(key, 3);

    assert_eq!(rbmap.get(&key), Some(&3));
}
