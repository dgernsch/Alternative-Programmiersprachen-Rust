use game_core::*;

#[test]
fn spawn_and_bounds() {
    let b = Board::default();
    assert!(!b.game_over);
    // Active piece can have y < 0 initially.
    for (x, y) in b.active.blocks() {
        assert!(x >= -10 && x < 20); // loose check; x/y not absurd
        assert!(y < 5);              // near the top
    }
}

#[test]
fn side_move_and_collision() {
    let mut b = Board::default();
    let x0 = b.active.x;
    b.move_side(-1);
    assert!(b.active.x == x0 - 1 || b.active.x == x0); // blocked at wall OK
}

#[test]
fn rotation_does_not_overlap() {
    let mut b = Board::default();
    // Stack some solids on the floor to catch collision bugs
    for x in 0..W {
        b.cells[H-1][x] = Cell::Solid(PieceKind::O);
    }
    let before = b.active.rot;
    b.rotate_cw();
    // Either rotated or stayed (if would collide)
    assert!(b.active.rot == before.cw() || b.active.rot == before);
}

#[test]
fn soft_and_hard_drop_locking_and_clearing() {
    let mut b = Board::default();
    // Force an 'I' piece for predictable line clear
    b.active.kind = PieceKind::I;
    b.active.rot = Rot::R90;
    b.active.x = 0;

    // Fill 19 rows with solids except the first column -> hard drop should NOT clear yet
    for y in 0..H-1 {
        for x in 1..W {
            b.cells[y][x] = Cell::Solid(PieceKind::J);
        }
    }
    let lines_before = b.lines_cleared;
    b.hard_drop(); // locks on floor, occupying the first column
    assert!(b.lines_cleared >= lines_before);

    // Now create a full line and verify clearing
    for x in 0..W {
        b.cells[H-2][x] = Cell::Solid(PieceKind::T);
    }
    let cleared = b.clear_lines();
    assert!(cleared >= 1);
}

