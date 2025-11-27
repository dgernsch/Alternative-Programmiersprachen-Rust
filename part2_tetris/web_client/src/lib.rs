use game_core::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let win = window().unwrap();
    let doc = win.document().unwrap();
    let canvas: HtmlCanvasElement = doc.get_element_by_id("game").unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let app = Rc::new(RefCell::new(App::new(ctx, canvas.width() as f64, canvas.height() as f64)));

    // Keyboard input
    {
        let doc = doc.clone();
        let app_k = Rc::clone(&app);
        let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
            let mut a = app_k.borrow_mut();
            match e.key().as_str() {
                "ArrowLeft"  => a.board.move_side(-1),
                "ArrowRight" => a.board.move_side(1),
                "ArrowUp"    => a.board.rotate_cw(),
                "ArrowDown"  => { a.want_soft_drop = true; }
                " "          => a.board.hard_drop(),
                "p" | "P"    => a.paused = !a.paused,
                _ => {}
            }
        });
        doc.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget(); // keep alive for app lifetime
    }

    // Game loop
    fn raf(f: &Closure<dyn FnMut()>) {
        window().unwrap().request_animation_frame(f.as_ref().unchecked_ref()).unwrap();
    }

    let mut last_ms = now_ms();
    let f = RcCell::new(None);
    let g = f.clone();
    let app_r = Rc::clone(&app);

    *g.borrow_mut() = Some(Closure::new(move || {
        let t = now_ms();
        let dt = (t - last_ms) as f64 / 1000.0;
        last_ms = t;

        {
            let mut a = app_r.borrow_mut();
            a.update(dt);
            a.draw();
        }

        raf(f.borrow().as_ref().unwrap());
    }));

    raf(g.borrow().as_ref().unwrap());
    Ok(())
}

struct RcCell<T>(Rc<RefCell<Option<T>>>);
impl<T> RcCell<T> {
    fn new(v: Option<T>) -> Self { Self(Rc::new(RefCell::new(v))) }
    fn clone(&self) -> Self { Self(self.0.clone()) }
    fn borrow(&self) -> std::cell::Ref<'_, Option<T>> { self.0.borrow() }
    fn borrow_mut(&self) -> std::cell::RefMut<'_, Option<T>> { self.0.borrow_mut() }
}

fn now_ms() -> f64 {
    js_sys::Date::now()
}

struct App {
    ctx: CanvasRenderingContext2d,
    w_px: f64,
    h_px: f64,
    cell: f64,
    pub board: Board,
    drop_timer: f64,
    drop_interval: f64,
    pub paused: bool,
    pub want_soft_drop: bool,
}

impl App {
    fn new(ctx: CanvasRenderingContext2d, w_px: f64, h_px: f64) -> Self {
        let cell = (w_px / W as f64).min(h_px / H as f64);
        Self {
            ctx, w_px, h_px, cell,
            board: Board::default(),
            drop_timer: 0.0,
            drop_interval: 0.8, // seconds per step, speeds up with lines
            paused: false,
            want_soft_drop: false,
        }
    }

    fn update(&mut self, dt: f64) {
        if self.paused || self.board.game_over { return; }

        // Speed curve: every 10 cleared lines, drop interval decreases
        let level = (self.board.lines_cleared / 10) as i32;
        self.drop_interval = (0.8_f64 - 0.05_f64 * level as f64).max(0.1);

        // Soft drop
        if self.want_soft_drop {
            let moved = self.board.soft_drop();
            if !moved { /* locked */ }
            self.want_soft_drop = false;
        }

        // Gravity
        self.drop_timer += dt;
        if self.drop_timer >= self.drop_interval {
            self.drop_timer = 0.0;
            self.board.soft_drop();
        }
    }

    fn draw(&mut self) {
        // clear
        self.ctx.set_fill_style_str(&"#0b0c10");
        self.ctx.fill_rect(0.0, 0.0, self.w_px, self.h_px);

        // settled cells
        for y in 0..H {
            for x in 0..W {
                if let Cell::Solid(k) = self.board.cells[y][x] {
                    self.draw_cell(x as i32, y as i32, k);
                }
            }
        }

        // active piece
        for (x, y) in self.board.active.blocks() {
            if y >= 0 {
                self.draw_cell(x, y, self.board.active.kind);
            }
        }

        if self.board.game_over {
            self.overlay_text("GAME OVER — press Space to drop again or reload");
        }
    }

    fn draw_cell(&self, x: i32, y: i32, k: PieceKind) {
        let (px, py) = (x as f64 * self.cell, y as f64 * self.cell);
        self.ctx.set_fill_style_str(&piece_color(k));
        self.ctx.fill_rect(px, py, self.cell - 1.0, self.cell - 1.0);
    }

    fn overlay_text(&self, msg: &str) {
        self.ctx.set_fill_style_str(&"rgba(0,0,0,0.5)");
        self.ctx.fill_rect(0.0, 0.0, self.w_px, self.h_px);
        self.ctx.set_fill_style_str(&"#fff");
        self.ctx.set_font("bold 20px system-ui");
        self.ctx.fill_text(msg, 10.0, self.h_px / 2.0).ok();
    }
}

fn piece_color(k: PieceKind) -> &'static str {
    match k {
        PieceKind::I => "#00FFFF",
        PieceKind::O => "#F7D154",
        PieceKind::T => "#B96AD9",
        PieceKind::S => "#3BD67F",
        PieceKind::Z => "#FF6B6B",
        PieceKind::J => "#5DA9E9",
        PieceKind::L => "#FF9F1C",
    }
}

