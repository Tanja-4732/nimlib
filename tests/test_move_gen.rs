use nimlib::{moves, NimAction, NimRule, NimSplit, Split, Stack, TakeSize};

#[test]
fn move_any_one() {
    for height in 0..=1000 {
        let rules = vec![NimRule {
            take: TakeSize::Any,
            split: Split::Never,
        }];

        let stacks = vec![Stack(height)];

        let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0));

        assert_eq!(moves.len(), height as usize);
    }
}

#[test]
fn move_any_two() {
    for height in 0..=1000 {
        let rules = vec![NimRule {
            take: TakeSize::Any,
            split: Split::Never,
        }];

        let stacks = vec![Stack(height), Stack(height)];

        let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0));

        assert_eq!(moves.len(), (height * 2) as usize);
    }
}

#[test]
fn move_any_five() {
    for height in 0..=1000 {
        let rules = vec![NimRule {
            take: TakeSize::Any,
            split: Split::Never,
        }];

        let stacks = vec![
            Stack(height),
            Stack(height),
            Stack(height),
            Stack(height),
            Stack(height),
        ];

        let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0));

        assert_eq!(moves.len(), (height * 5) as usize);
    }
}

#[test]
fn move_list_one() {
    for height in 0..=1000 {
        let rules = vec![NimRule {
            take: TakeSize::List(vec![1, 2, 3]),
            split: Split::Never,
        }];

        let stacks = vec![Stack(height)];

        let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0));

        assert_eq!(moves.len(), height.clamp(0, 3) as usize);
    }
}

#[test]
fn known_moves_simple() {
    let rules = vec![NimRule {
        take: TakeSize::List(vec![1, 2, 3]),
        split: Split::Never,
    }];

    let stacks = vec![Stack(10)];

    let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0))
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
    let rules = vec![NimRule {
        take: TakeSize::List(vec![1, 2, 3]),
        split: Split::Always,
    }];

    let stacks = vec![Stack(5)];

    let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0))
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
    let rules = vec![NimRule {
        take: TakeSize::List(vec![1, 2, 3, 7]),
        split: Split::Optional,
    }];

    let stacks = vec![Stack(5)];

    let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0))
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
    let rules = vec![
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
    ];

    let stacks = vec![Stack(0)];

    let moves = moves::calculate_legal_moves(&stacks, &rules, (0, 0));

    assert_eq!(moves.len(), 0);
}
