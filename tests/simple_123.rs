use nimlib::{algo, NimRule, Split, Stack, TakeSize};

#[test]
fn simple_123_game() {
    algo::clear_nimber_cache();

    let simple_rules: Vec<NimRule> = vec![NimRule {
        take: TakeSize::List(vec![1, 2, 3]),
        split: Split::Never,
    }];

    // Hand-verified nimbers
    assert_eq!(Stack(0).calculate_nimber(&simple_rules, 0), 0);
    assert_eq!(Stack(1).calculate_nimber(&simple_rules, 0), 1);
    assert_eq!(Stack(2).calculate_nimber(&simple_rules, 0), 2);
    assert_eq!(Stack(3).calculate_nimber(&simple_rules, 0), 3);
    assert_eq!(Stack(4).calculate_nimber(&simple_rules, 0), 0);
    assert_eq!(Stack(5).calculate_nimber(&simple_rules, 0), 1);
    assert_eq!(Stack(6).calculate_nimber(&simple_rules, 0), 2);
    assert_eq!(Stack(7).calculate_nimber(&simple_rules, 0), 3);
}
