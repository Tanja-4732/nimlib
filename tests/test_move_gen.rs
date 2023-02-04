use nimlib::{moves, NimAction, NimGame, NimRule, NimSplit, Split, Stack, TakeSize};

#[test]
fn move_any_one() {
    for height in 0..=1000 {
        let game = NimGame::new(
            vec![NimRule {
                take: TakeSize::Any,
                split: Split::Never,
            }],
            vec![Stack(height)],
        );

        let moves = moves::enumerate_moves(&game);

        assert_eq!(moves.len(), height as usize);
    }
}

#[test]
fn move_any_two() {
    for height in 0..=1000 {
        let game = NimGame::new(
            vec![NimRule {
                take: TakeSize::Any,
                split: Split::Never,
            }],
            vec![Stack(height), Stack(height)],
        );

        let moves = moves::enumerate_moves(&game);

        assert_eq!(moves.len(), (height * 2) as usize);
    }
}

#[test]
fn move_any_five() {
    for height in 0..=1000 {
        let game = NimGame::new(
            vec![NimRule {
                take: TakeSize::Any,
                split: Split::Never,
            }],
            vec![
                Stack(height),
                Stack(height),
                Stack(height),
                Stack(height),
                Stack(height),
            ],
        );

        let moves = moves::enumerate_moves(&game);

        assert_eq!(moves.len(), (height * 5) as usize);
    }
}

#[test]
fn move_list_one() {
    for height in 0..=1000 {
        let game = NimGame::new(
            vec![NimRule {
                take: TakeSize::List(vec![1, 2, 3]),
                split: Split::Never,
            }],
            vec![Stack(height)],
        );

        let moves = moves::enumerate_moves(&game);

        assert_eq!(moves.len(), height.clamp(0, 3) as usize);
    }
}

#[test]
fn known_moves_simple() {
    let game = NimGame::new(
        vec![NimRule {
            take: TakeSize::List(vec![1, 2, 3]),
            split: Split::Never,
        }],
        vec![Stack(10)],
    );

    let moves = moves::enumerate_moves(&game)
        .into_iter()
        .map(|mov| {
            if let NimAction::Take(take) = mov {
                take
            } else {
                panic!("Expected a take action");
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(moves.len(), 3);

    assert_eq!(moves[0].amount, 1);
    assert_eq!(moves[0].stack_index, 0);
    assert_eq!(moves[0].split, NimSplit::No);

    assert_eq!(moves[1].amount, 2);
    assert_eq!(moves[1].stack_index, 0);
    assert_eq!(moves[1].split, NimSplit::No);

    assert_eq!(moves[2].amount, 3);
    assert_eq!(moves[2].stack_index, 0);
    assert_eq!(moves[2].split, NimSplit::No);
}

#[test]
fn known_moves_split_always() {
    let game = NimGame::new(
        vec![NimRule {
            take: TakeSize::List(vec![1, 2, 3]),
            split: Split::Always,
        }],
        vec![Stack(5)],
    );

    let moves = moves::enumerate_moves(&game)
        .into_iter()
        .map(|mov| {
            if let NimAction::Take(take) = mov {
                take
            } else {
                panic!("Expected a take action");
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(moves.len(), 4);

    assert_eq!(moves[0].amount, 1);
    assert_eq!(moves[0].stack_index, 0);
    assert_eq!(moves[0].split, NimSplit::Yes(Stack(1), Stack(3)));

    assert_eq!(moves[1].amount, 1);
    assert_eq!(moves[1].stack_index, 0);
    assert_eq!(moves[1].split, NimSplit::Yes(Stack(2), Stack(2)));

    assert_eq!(moves[2].amount, 2);
    assert_eq!(moves[2].stack_index, 0);
    assert_eq!(moves[2].split, NimSplit::Yes(Stack(1), Stack(2)));

    assert_eq!(moves[3].amount, 3);
    assert_eq!(moves[3].stack_index, 0);
    assert_eq!(moves[3].split, NimSplit::Yes(Stack(1), Stack(1)));
}

#[test]
fn known_moves_split_optional() {
    let game = NimGame::new(
        vec![NimRule {
            take: TakeSize::List(vec![1, 2, 3, 7]),
            split: Split::Optional,
        }],
        vec![Stack(5)],
    );

    let moves = moves::enumerate_moves(&game)
        .into_iter()
        .map(|mov| {
            if let NimAction::Take(take) = mov {
                take
            } else {
                panic!("Expected a take action");
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(moves.len(), 7);

    // Take 1
    assert_eq!(moves[0].amount, 1);
    assert_eq!(moves[0].stack_index, 0);
    assert_eq!(moves[0].split, NimSplit::No);

    assert_eq!(moves[1].amount, 1);
    assert_eq!(moves[1].stack_index, 0);
    assert_eq!(moves[1].split, NimSplit::Yes(Stack(1), Stack(3)));

    assert_eq!(moves[2].amount, 1);
    assert_eq!(moves[2].stack_index, 0);
    assert_eq!(moves[2].split, NimSplit::Yes(Stack(2), Stack(2)));

    // Take 2
    assert_eq!(moves[3].amount, 2);
    assert_eq!(moves[3].stack_index, 0);
    assert_eq!(moves[3].split, NimSplit::No);

    assert_eq!(moves[4].amount, 2);
    assert_eq!(moves[4].stack_index, 0);
    assert_eq!(moves[4].split, NimSplit::Yes(Stack(1), Stack(2)));

    // Take 3
    assert_eq!(moves[5].amount, 3);
    assert_eq!(moves[5].stack_index, 0);
    assert_eq!(moves[5].split, NimSplit::No);

    assert_eq!(moves[6].amount, 3);
    assert_eq!(moves[6].stack_index, 0);
    assert_eq!(moves[6].split, NimSplit::Yes(Stack(1), Stack(1)));
}

#[test]
fn empty_position_many_rules() {
    let game = NimGame::new(
        vec![
            NimRule {
                take: TakeSize::Any,
                split: Split::Optional,
            },
            NimRule {
                take: TakeSize::List(vec![42]),
                split: Split::Always,
            },
            NimRule {
                take: TakeSize::List(vec![1, 2, 3]),
                split: Split::Optional,
            },
        ],
        vec![Stack(0)],
    );

    let moves = moves::enumerate_moves(&game);

    assert_eq!(moves.len(), 0);
}
