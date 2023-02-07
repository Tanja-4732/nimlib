use nimlib::{moves, NimGame, NimRule, Split, Stack, TakeSize};

#[test]
#[allow(deprecated)]
fn test_apply_move_1() {
    let rules = vec![NimRule {
        take: TakeSize::List(vec![1, 2, 3]),
        split: Split::Never,
    }];

    let stacks = vec![Stack(5)];

    let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0));

    // TODO maybe add more checks here
    assert_eq!(moves.len(), 3);

    let mut game = NimGame::new(rules, stacks);

    // Get the values before application and check them
    let (_rules, stacks, coins_a, coins_b) = nimlib::debug::get_inner_values_of_game(&game);

    assert_eq!(coins_a, 0);
    assert_eq!(coins_b, 0);

    assert_eq!(stacks[0], Stack(5));

    // Apply the move and check the values again
    moves::apply_move(&mut game, &moves[0]).expect("Failed to apply move");
    let (_rules, stacks, coins_a, coins_b) = nimlib::debug::get_inner_values_of_game(&game);

    assert_eq!(coins_a, 0);
    assert_eq!(coins_b, 0);

    assert_eq!(stacks[0], Stack(4));
}

#[test]
#[allow(deprecated)]
fn test_apply_move_2() {
    let rules = vec![NimRule {
        take: TakeSize::List(vec![3]),
        split: Split::Never,
    }];

    let stacks = vec![Stack(5)];

    let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0));

    // TODO maybe add more checks here
    assert_eq!(moves.len(), 1);

    let mut game = NimGame::new(rules, stacks);

    // Get the values before application and check them
    let (_rules, stacks, coins_a, coins_b) = nimlib::debug::get_inner_values_of_game(&game);

    assert_eq!(coins_a, 0);
    assert_eq!(coins_b, 0);

    assert_eq!(stacks[0], Stack(5));

    // Apply the move and check the values again
    moves::apply_move(&mut game, &moves[0]).expect("Failed to apply move");
    let (_rules, stacks, coins_a, coins_b) = nimlib::debug::get_inner_values_of_game(&game);

    assert_eq!(coins_a, 0);
    assert_eq!(coins_b, 0);

    assert_eq!(stacks[0], Stack(2));
}
