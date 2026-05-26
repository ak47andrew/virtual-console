use raylib::prelude::*;
use crate::consts::{RAM_SIZE, SCREEN_SIZE, TARGET_RESOLUTION};
use crate::emulator::emulator::Emulator;
use crate::emulator::memory::Memory;
use crate::shared::registers::{LongRegisters, Registers};

// ── Window & panel geometry ───────────────────────────────────────────────────

const SCALE: i32 = 3;
pub const DBG_W: i32 = SCREEN_SIZE.x;
pub const DBG_H: i32 = SCREEN_SIZE.y;

const SCREEN_W: i32 = TARGET_RESOLUTION.x * SCALE; // 768
const SCREEN_H: i32 = TARGET_RESOLUTION.y * SCALE; // 720
const REG_H: i32 = DBG_H - SCREEN_H;               // 240  (bottom-left panel)
const RIGHT_W: i32 = DBG_W - SCREEN_W;             // 512  (right column)
const HALF_H: i32 = DBG_H / 2;                     // 480  (each right panel)

// ── Hex-editor metrics ────────────────────────────────────────────────────────

const FONT_SZ: i32 = 14;
const CHAR_W: i32 = 8;   // approximate width of one glyph at FONT_SZ
const ROW_H: i32 = 18;
const BYTES_ROW: usize = 8;
const ADDR_W: i32 = CHAR_W * 8; // "000000: " = 8 chars

// ── Palette ───────────────────────────────────────────────────────────────────

const C_BG: Color       = Color { r: 18,  g: 18,  b: 28,  a: 255 };
const C_PANEL: Color    = Color { r: 26,  g: 26,  b: 38,  a: 255 };
const C_HEADER: Color   = Color { r: 38,  g: 38,  b: 56,  a: 255 };
const C_BORDER: Color   = Color { r: 60,  g: 60,  b: 90,  a: 255 };
const C_TEXT: Color     = Color { r: 200, g: 200, b: 210, a: 255 };
const C_DIM: Color      = Color { r: 90,  g: 90,  b: 115, a: 255 };
const C_PC: Color       = Color { r: 255, g: 220, b: 0,   a: 255 };
const C_PC_BG: Color    = Color { r: 55,  g: 45,  b: 0,   a: 255 };
const C_HOVER_BG: Color = Color { r: 55,  g: 55,  b: 90,  a: 255 };
const C_TOG_ON: Color   = Color { r: 50,  g: 170, b: 70,  a: 255 };
const C_TOG_OFF: Color  = Color { r: 50,  g: 50,  b: 75,  a: 255 };
const C_OVERLAY: Color  = Color { r: 0,   g: 0,   b: 0,   a: 150 };
const C_TIP_BG: Color   = Color { r: 8,   g: 8,   b: 18,  a: 220 };

// ── Input bitmask ─────────────────────────────────────────────────────────────

const TOGGLE_DEFS: [(&str, u8); 8] = [
    ("UP",    1 << 0),
    ("DOWN",  1 << 1),
    ("LEFT",  1 << 2),
    ("RIGHT", 1 << 3),
    ("Z",     1 << 4),
    ("X",     1 << 5),
    ("C",     1 << 6),
    ("ESC",   1 << 7),
];

// Toggle button geometry (relative to registers panel origin)
const TOG_W: i32 = 54;
const TOG_H: i32 = 26;
const TOG_GAP: i32 = 5;
const TOG_X0: i32 = 8;
const TOG_Y_OFFSET: i32 = 160; // from panel top

// ─────────────────────────────────────────────────────────────────────────────

pub struct Debugger {
    emulator: Emulator,
    pub paused: bool,
    ram_scroll: usize,
    rom_scroll: usize,
    pub input_toggles: u8,
    hovered_byte: Option<usize>,
}

impl Debugger {
    pub fn new(emulator: Emulator) -> Self {
        Self {
            emulator,
            paused: true,
            ram_scroll: 0,
            rom_scroll: 0,
            input_toggles: 0,
            hovered_byte: None,
        }
    }

    // ── Called each frame before drawing ─────────────────────────────────────

    /// Returns true if the emulator should execute one step this frame.
    pub fn update(&mut self, rl: &RaylibHandle) -> bool {
        let mouse = rl.get_mouse_position();
        let wheel = rl.get_mouse_wheel_move();

        // Scroll ROM panel (right column, bottom half)
        if mouse.x >= SCREEN_W as f32 && mouse.y >= HALF_H as f32 && wheel != 0.0 {
            let rom_rows = (self.emulator.memory.total_size() - Memory::rom_start()) / BYTES_ROW;
            let max = rom_rows.saturating_sub(1);
            let mut s = self.rom_scroll;
            Self::scroll_val(&mut s, wheel, max);
            self.rom_scroll = s;
        }

        // RAM scroll (needs to be done separately due to borrow)
        if mouse.x >= SCREEN_W as f32 && mouse.y < HALF_H as f32 && wheel != 0.0 {
            let max = (RAM_SIZE / BYTES_ROW).saturating_sub(1);
            let mut s = self.ram_scroll;
            Self::scroll_val(&mut s, wheel, max);
            self.ram_scroll = s;
        }

        // Input toggle clicks
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.handle_toggle_click(mouse);
        }

        // Update hover
        self.hovered_byte = self.resolve_hover(mouse);

        // Step on SPACE (only when paused)
        self.paused && rl.is_key_pressed(KeyboardKey::KEY_SPACE)
    }

    /// Called by the emulator when it executes a Vsync opcode.
    /// Grabs toggle state → writes INPUT_HELD / INPUT_PRESSED → clears toggles.
    pub fn on_vsync(&mut self) {
        let prev = self.emulator.memory.peek(Memory::input_held(), 1)[0];
        let curr = self.input_toggles;
        let pressed = curr & !prev;
        self.emulator.memory.put(Memory::input_held(), &[curr]);
        self.emulator.memory.put(Memory::input_pressed(), &[pressed]);
        self.input_toggles = 0;
    }

    // ── Drawing ───────────────────────────────────────────────────────────────

    pub fn draw(
        &self,
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
        mouse: Vector2
    ) {
        d.clear_background(C_BG);
        self.draw_screen_panel(d, texture);
        self.draw_registers_panel(d);
        self.draw_hex_panel(d, mouse,
                            SCREEN_W, 0, RIGHT_W, HALF_H,
                            "RAM",
                            Memory::vram_size(), RAM_SIZE,
                            self.ram_scroll, None,
        );
        let pc = self.emulator.memory.read_pc();
        let rom_start = Memory::rom_start();
        let rom_size = self.emulator.memory.total_size() - rom_start;
        self.draw_hex_panel(d, mouse,
                            SCREEN_W, HALF_H, RIGHT_W, HALF_H,
                            "ROM",
                            rom_start, rom_size,
                            self.rom_scroll, Some(pc),
        );
    }

    // ── Screen panel (top-left) ───────────────────────────────────────────────

    fn draw_screen_panel(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        d.draw_rectangle(0, 0, SCREEN_W, SCREEN_H, C_PANEL);

        d.draw_texture_pro(
            texture,
            Rectangle::new(0.0, 0.0,
                           TARGET_RESOLUTION.x as f32,
                           TARGET_RESOLUTION.y as f32),
            Rectangle::new(0.0, 0.0, SCREEN_W as f32, SCREEN_H as f32),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        d.draw_rectangle_lines(0, 0, SCREEN_W, SCREEN_H, C_BORDER);
    }

    // ── Registers + input toggles panel (bottom-left) ────────────────────────

    fn draw_registers_panel(&self, d: &mut RaylibDrawHandle) {
        let (px, py) = (0, SCREEN_H);

        d.draw_rectangle(px, py, SCREEN_W, REG_H, C_PANEL);
        d.draw_rectangle(px, py, SCREEN_W, ROW_H, C_HEADER);
        d.draw_text("REGISTERS", px + 4, py + 2, FONT_SZ, C_TEXT);
        d.draw_rectangle_lines(px, py, SCREEN_W, REG_H, C_BORDER);

        let mut x = px + 8;
        let mut y = py + ROW_H + 4;

        // ── 8-bit registers ──
        d.draw_text("8-bit:", x, y, FONT_SZ - 2, C_DIM);
        y += ROW_H;

        let col_w = CHAR_W * 18; // "G1: FFh  255 " ≈ 18 chars
        for reg in Registers::all() {
            let val = self.emulator.memory.read_reg(reg);
            let label = format!("{:>2}: {:02X}h {:3}", reg_name_8(&reg), val, val);
            d.draw_text(&label, x, y, FONT_SZ, C_TEXT);
            x += col_w;
            if x + col_w > px + SCREEN_W {
                x = px + 8;
                y += ROW_H;
            }
        }

        x = px + 8;
        y += ROW_H + 4;

        // ── 64-bit registers ──
        d.draw_text("64-bit:", x, y, FONT_SZ - 2, C_DIM);
        y += ROW_H;

        let col64_w = CHAR_W * 28; // "PC : 0000000000000000h " ≈ 28 chars
        for reg in LongRegisters::all() {
            let val = self.emulator.memory.read_reg_long(reg);
            let label = format!("{}: {:016X}h", reg_name_64(&reg), val);
            d.draw_text(&label, x, y, FONT_SZ, C_TEXT);
            x += col64_w;
            if x + col64_w > px + SCREEN_W {
                x = px + 8;
                y += ROW_H;
            }
        }

        // ── Input toggles ──
        let ty = py + TOG_Y_OFFSET;
        d.draw_text(
            "INPUT TOGGLES  (click to arm, applied + cleared on vsync):",
            px + 8, ty - ROW_H, FONT_SZ - 2, C_DIM,
        );

        for (i, (label, bit)) in TOGGLE_DEFS.iter().enumerate() {
            let tx = px + TOG_X0 + i as i32 * (TOG_W + TOG_GAP);
            let on = (self.input_toggles & bit) != 0;
            let bg = if on { C_TOG_ON } else { C_TOG_OFF };
            d.draw_rectangle(tx, ty, TOG_W, TOG_H, bg);
            d.draw_rectangle_lines(tx, ty, TOG_W, TOG_H, C_BORDER);
            let lw = measure_text(label, FONT_SZ - 2);
            d.draw_text(
                label,
                tx + (TOG_W - lw) / 2,
                ty + (TOG_H - (FONT_SZ - 2)) / 2,
                FONT_SZ - 2,
                Color::WHITE,
            );
        }
    }

    // ── Generic hex-editor panel ──────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    fn draw_hex_panel(
        &self,
        d: &mut RaylibDrawHandle,
        mouse: Vector2,
        px: i32, py: i32, pw: i32, ph: i32,
        title: &str,
        region_start: usize,
        region_size: usize,
        scroll: usize,
        pc: Option<usize>,
    ) {
        d.draw_rectangle(px, py, pw, ph, C_PANEL);
        d.draw_rectangle(px, py, pw, ROW_H, C_HEADER);
        d.draw_text(
            &format!("{title}  ({region_size} bytes, scroll: {scroll})"),
            px + 4, py + 2, FONT_SZ, C_TEXT,
        );
        d.draw_rectangle_lines(px, py, pw, ph, C_BORDER);

        let visible = ((ph - ROW_H) / ROW_H) as usize;
        let total_rows = (region_size + BYTES_ROW - 1) / BYTES_ROW;
        let end_row = (scroll + visible).min(total_rows);

        for row in scroll..end_row {
            let row_base = region_start + row * BYTES_ROW;
            let ry = py + ROW_H + (row - scroll) as i32 * ROW_H;

            // Address
            d.draw_text(
                &format!("{:06X}: ", row_base),
                px + 2, ry + 1, FONT_SZ, C_DIM,
            );

            for col in 0..BYTES_ROW {
                let addr = row_base + col;
                if addr >= region_start + region_size { break; }

                let byte = self.emulator.memory.peek(addr, 1)[0];
                let bx = px + ADDR_W + col as i32 * CHAR_W * 3;

                let is_pc = pc.map_or(false, |p| p == addr);
                let is_hov = self.hovered_byte.map_or(false, |h| h == addr);

                // Background highlight
                if is_pc {
                    d.draw_rectangle(bx - 1, ry, CHAR_W * 2 + 2, ROW_H - 1, C_PC_BG);
                    d.draw_rectangle_lines(bx - 1, ry, CHAR_W * 2 + 2, ROW_H - 1, C_PC);
                } else if is_hov {
                    d.draw_rectangle(bx - 1, ry, CHAR_W * 2 + 2, ROW_H - 1, C_HOVER_BG);
                }

                let color = if is_pc { C_PC } else if byte == 0 { C_DIM } else { C_TEXT };
                d.draw_text(&format!("{:02X}", byte), bx, ry + 1, FONT_SZ, color);
            }
        }

        // Tooltip for hovered byte
        if let Some(addr) = self.hovered_byte {
            if addr >= region_start && addr < region_start + region_size {
                // Only show tooltip when mouse is actually over this panel
                let mx = mouse.x as i32;
                let my = mouse.y as i32;
                if mx >= px && mx < px + pw && my >= py && my < py + ph {
                    let byte = self.emulator.memory.peek(addr, 1)[0];
                    let tip = format!(" 0x{:X}", addr);
                    let tip_w = measure_text(&tip, FONT_SZ - 1);
                    let tx = (mx + 14).min(px + pw - tip_w - 4);
                    let ty = (my - ROW_H - 2).max(py + ROW_H);
                    d.draw_rectangle(tx - 2, ty - 2, tip_w + 4, ROW_H + 2, C_TIP_BG);
                    d.draw_rectangle_lines(tx - 2, ty - 2, tip_w + 4, ROW_H + 2, C_BORDER);
                    d.draw_text(&tip, tx, ty, FONT_SZ - 1, C_TEXT);
                }
            }
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn scroll_val(s: &mut usize, wheel: f32, max: usize) {
        if wheel < 0.0 {
            *s = (*s + 3).min(max);
        } else {
            *s = s.saturating_sub(3);
        }
    }

    fn handle_toggle_click(&mut self, mouse: Vector2) {
        let panel_y = SCREEN_H;
        let ty = panel_y + TOG_Y_OFFSET;
        for (i, (_, bit)) in TOGGLE_DEFS.iter().enumerate() {
            let tx = TOG_X0 + i as i32 * (TOG_W + TOG_GAP);
            if mouse.x >= tx as f32 && mouse.x < (tx + TOG_W) as f32
                && mouse.y >= ty as f32 && mouse.y < (ty + TOG_H) as f32
            {
                self.input_toggles ^= bit;
                return;
            }
        }
    }

    fn resolve_hover(&self, mouse: Vector2) -> Option<usize> {
        // RAM panel
        if let Some(addr) = self.hex_hover(mouse,
                                           SCREEN_W, 0, RIGHT_W, HALF_H,
                                           Memory::vram_size(), RAM_SIZE, self.ram_scroll,
        ) { return Some(addr); }

        // ROM panel
        let rom_start = Memory::rom_start();
        let rom_size = self.emulator.memory.total_size() - rom_start;
        self.hex_hover(mouse,
                       SCREEN_W, HALF_H, RIGHT_W, HALF_H,
                       rom_start, rom_size, self.rom_scroll,
        )
    }

    fn hex_hover(
        &self,
        mouse: Vector2,
        px: i32, py: i32, pw: i32, ph: i32,
        region_start: usize, region_size: usize,
        scroll: usize,
    ) -> Option<usize> {
        let mx = mouse.x as i32;
        let my = mouse.y as i32;

        if mx < px || mx >= px + pw { return None; }
        if my < py + ROW_H || my >= py + ph { return None; }

        let row = (my - py - ROW_H) / ROW_H + scroll as i32;
        let bx = mx - px - ADDR_W;
        if bx < 0 { return None; }

        let col = bx / (CHAR_W * 3);
        if col >= BYTES_ROW as i32 { return None; }

        let addr = region_start + row as usize * BYTES_ROW + col as usize;
        if addr < region_start + region_size { Some(addr) } else { None }
    }
}

// ── Register name helpers ─────────────────────────────────────────────────────

fn reg_name_8(reg: &Registers) -> &'static str {
    match reg {
        Registers::A => "A",
        Registers::X => "X",
        Registers::Y => "Y",
        Registers::Z => "Z",
        Registers::G1 => "G1",
        Registers::G2 => "G2",
        Registers::G3 => "G3",
        Registers::G4 => "G4",
        Registers::G5 => "G5",
    }
}

fn reg_name_64(reg: &LongRegisters) -> &'static str {
    match reg {
        LongRegisters::PC  => "PC ",
        LongRegisters::LL1 => "LL1",
        LongRegisters::LL2 => "LL2",
        LongRegisters::GP1 => "GP1",
        LongRegisters::GP2 => "GP2",
        LongRegisters::GP3 => "GP3",
    }
}

fn measure_text(string: &str, fontsize: i32) -> i32 {
    (string.len() as f64 * (fontsize as f64 / 1.7)) as i32
}

pub fn entry_debugger(mut rl: RaylibHandle, mut thread: RaylibThread, mut emulator: Emulator) {
    let mut debugger = Debugger::new(emulator);
    let mut texture = rl.load_texture_from_image(&thread,
                                                 &Image::gen_image_color(TARGET_RESOLUTION.x, TARGET_RESOLUTION.y, Color::BLACK)).unwrap();
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    while !rl.window_should_close() {
        let mouse = rl.get_mouse_position();

        if debugger.update(&rl) {
            debugger.emulator.step()
        }

        if debugger.emulator.update_texture {
            debugger.on_vsync()
        }

        debugger.emulator.new_frame(&mut texture, &rl);
        let mut d = rl.begin_drawing(&thread);

        debugger.draw(&mut d, &mut texture, mouse);
    }
}
