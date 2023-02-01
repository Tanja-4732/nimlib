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
