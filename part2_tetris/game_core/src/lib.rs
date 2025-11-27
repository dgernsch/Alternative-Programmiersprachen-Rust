//! Tiny Tetris core: board, pieces, rules. No WASM/JS here.
//!
//! Board is 10 x 20 (visible). Coordinates: (x,y), x:0..10, y:0..20 (0 at top).

use rand::{seq::SliceRandom, rng};

pub const W: usize = 10;
pub const H: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Solid(PieceKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind { I, O, T, S, Z, J, L }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rot { R0, R90, R180, R270 }

impl Rot {
    pub fn cw(self) -> Self {
        match self { Rot::R0=>Rot::R90, Rot::R90=>Rot::R180, Rot::R180=>Rot::R270, Rot::R270=>Rot::R0 }
    }
    pub fn ccw(self) -> Self { self.cw().cw().cw() }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub rot: Rot,
    pub x: i32, // top-left of the 4x4 local grid
    pub y: i32,
}

impl Piece {
    pub fn spawn(kind: PieceKind) -> Self {
        // Spawn near the top; x so that 4x4 fits centered-ish
        Self { kind, rot: Rot::R0, x: 3, y: -1 }
    }

    /// Return the 4 blocks that make up this piece in board coords.
    pub fn blocks(&self) -> [(i32, i32); 4] {
        // TODO: return coordinates for the active rotation.
        // Represent each tetromino in a 4x4 local grid, then rotate.
        // Hints:
        //  - Define each shape at R0 as a list of (lx, ly) in 0..4
        //  - Apply rotation:
        //      R0: (x,y)
        //      R90: (3-y, x)
        //      R180: (3-x, 3-y)
        //      R270: (y, 3-x)
        //  - Translate by (self.x, self.y)
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub cells: [[Cell; W]; H],
    pub active: Piece,
    pub bag: Vec<PieceKind>,
    pub next: PieceKind,
    pub score: u32,
    pub lines_cleared: u32,
    pub game_over: bool,
}

impl Default for Board {
    fn default() -> Self {
        let mut b = Self {
            cells: [[Cell::Empty; W]; H],
            active: Piece::spawn(PieceKind::T),
            bag: Vec::new(),
            next: PieceKind::I,
            score: 0,
            lines_cleared: 0,
            game_over: false,
        };
        b.refill_bag();
        b.spawn_new_active();
        b
    }
}

impl Board {
    fn refill_bag(&mut self) {
        if self.bag.is_empty() {
            self.bag = vec![
                PieceKind::I, PieceKind::O, PieceKind::T, PieceKind::S,
                PieceKind::Z, PieceKind::J, PieceKind::L,
            ];
            self.bag.shuffle(&mut rng());
        }
    }

    fn draw_from_bag(&mut self) -> PieceKind {
        self.refill_bag();
        self.bag.pop().unwrap()
    }

    fn spawn_new_active(&mut self) {
        self.next = self.draw_from_bag();
        self.active = Piece::spawn(self.next);
        // If it collides immediately -> game over
        if self.collides(&self.active) {
            self.game_over = true;
        }
    }

    fn in_bounds(x: i32, y: i32) -> bool {
        x >= 0 && x < W as i32 && y < H as i32
    }

    pub fn collides(&self, p: &Piece) -> bool {
        // TODO: true if any block is out of bounds (y<0 is allowed until lock) or hits Solid.
        // Treat y<0 as *allowed* (spawning above the board), but x bounds must hold.
        // Once y>=0, check cell occupancy.
        unimplemented!()
    }

    pub fn move_side(&mut self, dx: i32) {
        if self.game_over { return; }
        let mut np = self.active.clone();
        np.x += dx;
        if !self.collides(&np) {
            self.active = np;
        }
    }

    pub fn rotate_cw(&mut self) {
        if self.game_over { return; }
        let mut np = self.active.clone();
        np.rot = np.rot.cw();
        // simple: try rotated, else keep original (no wall kicks in v1)
        if !self.collides(&np) {
            self.active = np;
        }
    }

    pub fn soft_drop(&mut self) -> bool {
        // returns true if piece moved down
        if self.game_over { return false; }
        let mut np = self.active.clone();
        np.y += 1;
        if !self.collides(&np) {
            self.active = np;
            true
        } else {
            self.lock_active();
            false
        }
    }

    pub fn hard_drop(&mut self) {
        if self.game_over { return; }
        while self.soft_drop() {}
    }

    fn lock_active(&mut self) {
        // TODO: convert active blocks into Solid cells if y>=0
        // Then clear complete lines, update score/lines, and spawn next piece.
        unimplemented!()
    }

    pub fn clear_lines(&mut self) -> u32 {
        // TODO: remove full rows; return how many were cleared.
        // Strategy: collect rows to keep, then fill from bottom.
        unimplemented!()
    }
}

