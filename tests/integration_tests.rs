use vibe_tinyurl::utils::ShortUrlUtil;

#[test]
fn test_short_url_util_base62_roundtrip() {
    let util = ShortUrlUtil::new(6);
    for i in 0..1000 {
        let encoded = util.base10_to_base62(i).unwrap();
        let decoded = util.base62_to_base10(&encoded).unwrap();
        assert_eq!(i, decoded, "Roundtrip failed for {}", i);
    }
}

#[test]
fn test_short_url_util_random_length() {
    for len in 1..=6 {
        let util = ShortUrlUtil::new(len);
        let s = util.random_str();
        assert_eq!(s.len(), len, "Random string wrong length for {}", len);
    }
}

#[test]
fn test_short_url_util_max_values() {
    // Test that we can handle the maximum values for each length
    let util_1 = ShortUrlUtil::new(1);
    assert_eq!(util_1.max_num(), 61); // 62^1 - 1

    let util_2 = ShortUrlUtil::new(2);
    assert_eq!(util_2.max_num(), 3843); // 62^2 - 1

    let util_6 = ShortUrlUtil::new(6);
    assert_eq!(util_6.max_num(), 56800235583); // 62^6 - 1
}
