use minifb::{Key, Window, WindowOptions};

pub enum Opcode {
    Halt = 0x00,
    LoadImm = 0x01,
    VecAdd = 0x02,
    VecMul = 0x03,
    Broadcast = 0x04,
    AssertContract = 0x05,
    UIRender = 0x08,
    UILayout = 0x09,
    UIComponent = 0x0A,
    WinCreate = 0x0E,
    WinUpdate = 0x0F,
    EventGet = 0x10,
    DrawRect = 0x11,
    WinPoll = 0x12,
    DrawText = 0x13,
    InputGet = 0x14, ScreenCap = 0x15, StreamCap = 0x16, AsmBlock = 0x17, VolatileWrite = 0x18, VolatileRead = 0x19, AtomicOp = 0x1A, UIHologramStart = 0x1B, UIHologramEnd = 0x1C, UIPostProcess = 0x1D, UINeuroAdapt = 0x1E,
}

// Minimal 8x8 bitmap font for "Living GUI" (ASCII 32-126)
const FONT: [[u8; 8]; 82] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Space
    [0x18, 0x3c, 0x3c, 0x18, 0x18, 0x00, 0x18, 0x00], // !
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x3c, 0x66, 0x6e, 0x7e, 0x76, 0x66, 0x3c, 0x00], // 0 (Pseudo)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // : (58)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x3c, 0x66, 0x6e, 0x7e, 0x76, 0x66, 0x3c, 0x00], // @
    [0x18, 0x3c, 0x66, 0x66, 0x7e, 0x66, 0x66, 0x00], // A
    [0x7c, 0x66, 0x66, 0x7c, 0x66, 0x66, 0x7c, 0x00], // B
    [0x3c, 0x66, 0x06, 0x06, 0x06, 0x66, 0x3c, 0x00], // C
    [0x78, 0x6c, 0x66, 0x66, 0x66, 0x6c, 0x78, 0x00], // D
    [0x7e, 0x06, 0x06, 0x3e, 0x06, 0x06, 0x7e, 0x00], // E
    [0x7e, 0x06, 0x06, 0x3e, 0x06, 0x06, 0x06, 0x00], // F
    [0x3c, 0x66, 0x06, 0x56, 0x66, 0x66, 0x3c, 0x00], // G
    [0x66, 0x66, 0x66, 0x7e, 0x66, 0x66, 0x66, 0x00], // H
    [0x3c, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3c, 0x00], // I
    [0x1c, 0x0c, 0x0c, 0x0c, 0x6c, 0x6c, 0x38, 0x00], // J
    [0x66, 0x6c, 0x78, 0x70, 0x78, 0x6c, 0x66, 0x00], // K
    [0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x7e, 0x00], // L
    [0x63, 0x77, 0x7f, 0x6b, 0x63, 0x63, 0x63, 0x00], // M
    [0x66, 0x76, 0x7e, 0x7e, 0x6e, 0x66, 0x66, 0x00], // N
    [0x3c, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3c, 0x00], // O
    [0x7c, 0x66, 0x66, 0x7c, 0x06, 0x06, 0x06, 0x00], // P
    [0x3c, 0x66, 0x66, 0x66, 0x66, 0x3c, 0x30, 0x00], // Q
    [0x7c, 0x66, 0x66, 0x7c, 0x6c, 0x66, 0x66, 0x00], // R
    [0x3c, 0x66, 0x06, 0x3c, 0x60, 0x66, 0x3c, 0x00], // S
    [0x7e, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00], // T
    [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3c, 0x00], // U
    [0x66, 0x66, 0x66, 0x66, 0x66, 0x3c, 0x18, 0x00], // V
    [0x63, 0x63, 0x63, 0x6b, 0x7f, 0x77, 0x63, 0x00], // W
    [0x66, 0x66, 0x3c, 0x18, 0x3c, 0x66, 0x66, 0x00], // X
    [0x66, 0x66, 0x66, 0x3c, 0x18, 0x18, 0x18, 0x00], // Y
    [0x7e, 0x60, 0x30, 0x18, 0x0c, 0x06, 0x7e, 0x00], // Z
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
];

pub struct Particle { pub x: f32, pub y: f32, pub vx: f32, pub vy: f32, pub life: f32, }

pub struct JUR {
    stack: Vec<f32>,
    window: Option<Window>,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    _layout_y: usize,
    _layout_x: usize,
    input_buffer: Vec<char>,
    is_recording: bool,
    ffmpeg_child: Option<std::process::Child>, particles: Vec<Particle>, parallax_offset: (f32, f32), alpha_modifier: f32, is_hologram: bool,
}

impl JUR {
    fn blend(&self, bg: u32, fg: u32, alpha: f32) -> u32 { let r1 = ((bg >> 16) & 0xFF) as f32; let g1 = ((bg >> 8) & 0xFF) as f32; let b1 = (bg & 0xFF) as f32; let r2 = ((fg >> 16) & 0xFF) as f32; let g2 = ((fg >> 8) & 0xFF) as f32; let b2 = (fg & 0xFF) as f32; let r = (r1 * (1.0 - alpha) + r2 * alpha) as u32; let g = (g1 * (1.0 - alpha) + g2 * alpha) as u32; let b = (b1 * (1.0 - alpha) + b2 * alpha) as u32; (r << 16) | (g << 8) | b }
    fn apply_glitch_fx(&mut self, intensity: f32) { if self.buffer.is_empty() { return; } let shift = (intensity * 10.0) as usize; let mut new_buf = self.buffer.clone(); for i in shift..self.buffer.len()-shift { let r = (self.buffer[i - shift] >> 16) & 0xFF; let g = (self.buffer[i] >> 8) & 0xFF; let b = self.buffer[i + shift] & 0xFF; new_buf[i] = (r << 16) | (g << 8) | b; } for y in (0..self.height).step_by(2) { for x in 0..self.width { let idx = y * self.width + x; let p = new_buf[idx]; let r = ((p >> 16) & 0xFF) / 2; let g = ((p >> 8) & 0xFF) / 2; let b = (p & 0xFF) / 2; new_buf[idx] = (r << 16) | (g << 8) | b; } } self.buffer = new_buf; }
    fn update_particles(&mut self) { let w = self.width as f32; let h = self.height as f32; for p in &mut self.particles { p.x += p.vx; p.y += p.vy; p.life -= 0.01; if p.x < 0.0 || p.x >= w || p.y < 0.0 || p.y >= h { p.life = 0.0; } } self.particles.retain(|p| p.life > 0.0); for p in &self.particles { let idx = (p.y as usize) * self.width + (p.x as usize); if idx < self.buffer.len() { self.buffer[idx] = 0x00f0ff; } } }
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            window: None,
            buffer: Vec::new(),
            width: 0,
            height: 0,
            _layout_y: 0,
            _layout_x: 0,
            input_buffer: Vec::new(),
            is_recording: false,
            ffmpeg_child: None, particles: Vec::new(), parallax_offset: (0.0, 0.0), alpha_modifier: 1.0, is_hologram: false,
        }
    }

    fn save_bmp(&self, filename: &str) {
        use std::fs::File;
        use std::io::Write;
        if self.buffer.is_empty() { return; }
        
        let mut f = File::create(filename).expect("Failed to create screenshot file");
        let filesize = 54 + (self.width * self.height * 3) as u32;
        
        // BMP Header
        f.write_all(b"BM").unwrap();
        f.write_all(&filesize.to_le_bytes()).unwrap();
        f.write_all(&[0; 4]).unwrap();
        f.write_all(&54u32.to_le_bytes()).unwrap();
        
        // DIB Header
        f.write_all(&40u32.to_le_bytes()).unwrap();
        f.write_all(&(self.width as u32).to_le_bytes()).unwrap();
        f.write_all(&(-(self.height as i32)).to_le_bytes()).unwrap(); // Top-down
        f.write_all(&1u16.to_le_bytes()).unwrap();
        f.write_all(&24u16.to_le_bytes()).unwrap();
        f.write_all(&[0; 24]).unwrap();
        
        let mut pixels = Vec::with_capacity(self.width * self.height * 3);
        for &p in &self.buffer {
            pixels.push((p & 0xFF) as u8);       // B
            pixels.push(((p >> 8) & 0xFF) as u8);  // G
            pixels.push(((p >> 16) & 0xFF) as u8); // R
        }
        f.write_all(&pixels).unwrap();
        println!("[UDS] Screenshot saved: {}", filename);
    }

    fn draw_char(&mut self, c: char, x: usize, y: usize, color: u32) {
        let idx = (c as usize).saturating_sub(32).min(FONT.len() - 1);
        let glyph = FONT[idx];
        for row in 0..8 {
            for col in 0..8 {
                if (glyph[row] & (1 << (7 - col))) != 0 {
                    let (ox, oy) = if self.is_hologram { (self.parallax_offset.0 as i32, self.parallax_offset.1 as i32) } else { (0, 0) }; let px = (x as i32 + col as i32 + ox) as usize;
                    let py = (y as i32 + row as i32 + oy) as usize;
                    if px < self.width && py < self.height {
                        if self.alpha_modifier < 1.0 { let bg = self.buffer[py * self.width + px]; self.buffer[py * self.width + px] = self.blend(bg, color, self.alpha_modifier); } else { self.buffer[py * self.width + px] = color; }
                    }
                }
            }
        }
    }

    pub fn run(&mut self, code: &[u8]) {
        let mut ip = 0;
        while ip < code.len() {
            let op = code[ip];
            match op {
                0x01 => { // LoadImm
                    let val = f32::from_le_bytes(code[ip+1..ip+5].try_into().unwrap());
                    self.stack.push(val);
                    ip += 4;
                }
                0x0E => { // WinCreate
                    let h = self.stack.pop().unwrap_or(600.0) as usize;
                    let w = self.stack.pop().unwrap_or(800.0) as usize;
                    self.width = w;
                    self.height = h;
                    
                    self.buffer = vec![0x050506; w * h];
                    for row in (0..h).step_by(4) {
                        for col in 0..w {
                            self.buffer[row * w + col] = 0x0a0a0b;
                        }
                    }
                    
                    let headless = std::env::var("JARVIS_HEADLESS").is_ok();

                    if headless {
                        println!("[UDS] Headless Mode Forced via ENV.");
                        println!("[UDS] Virtual Framebuffer Initialized: {}x{}", w, h);
                    } else {
                        let win_res = Window::new(
                            "Jarvis Universal Runtime (JUR) - Production",
                            w,
                            h,
                            WindowOptions::default(),
                        );

                        match win_res {
                            Ok(mut win) => {
                                win.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
                                self.window = Some(win);
                                println!("[UDS] Physical Window Created: {}x{}", w, h);
                            }
                            Err(_) => {
                                println!("[UDS] Headless Fallback: Physical display unavailable.");
                                println!("[UDS] Virtual Framebuffer Initialized: {}x{}", w, h);
                            }
                        }
                    }
                }
                0x11 => { // DrawRect
                    let color = self.stack.pop().unwrap_or(0.0).to_bits();
                    let h = self.stack.pop().unwrap_or(10.0) as usize;
                    let w = self.stack.pop().unwrap_or(10.0) as usize;
                    let y = self.stack.pop().unwrap_or(0.0) as usize;
                    let x = self.stack.pop().unwrap_or(0.0) as usize;
                    
                    for row in y..(y+h).min(self.height) {
                        for col in x..(x+w).min(self.width) {
                            self.buffer[row * self.width + col] = color;
                        }
                    }
                }
                0x0A => { // UIComponent
                    let kind_hash = self.stack.pop().unwrap_or(0.0) as u32;
                    let _arg = self.stack.pop().unwrap_or(0.0);
                    
                    #[allow(unused_assignments)]
                    let mut x = 0; 
                    #[allow(unused_assignments)]
                    let mut y = 0; 
                    #[allow(unused_assignments)]
                    let mut w = 0; 
                    #[allow(unused_assignments)]
                    let mut h = 0;
                    let mut color = 0x121214; 
                    let mut border_color = 0x00f0ff;
                    let mut draw_notches = true;

                    match kind_hash {
                        585 => { // Header
                            x = 0; y = 0; w = self.width; h = 60;
                            border_color = 0x00f0ff; draw_notches = false;
                            for r in 25..35 { for c in 20..30 { self.buffer[r * self.width + c] = 0x00ff00; } }
                            self.draw_char('J', 40, 25, 0xffffff);
                            self.draw_char('A', 50, 25, 0xffffff);
                            self.draw_char('R', 60, 25, 0xffffff);
                            self.draw_char('V', 70, 25, 0xffffff);
                            self.draw_char('I', 80, 25, 0xffffff);
                            self.draw_char('S', 90, 25, 0xffffff);
                        }
                        875 => { // VideoFeed
                            x = 20; y = 80; w = self.width - 340; h = self.height - 180;
                            border_color = 0x00f0ff;
                            self.draw_char('V', x+10, y+10, 0xffffff);
                            self.draw_char('I', x+20, y+10, 0xffffff);
                            self.draw_char('D', x+30, y+10, 0xffffff);
                        }
                        1121 => { // MessageList
                            x = self.width - 300; y = 80; w = 280; h = self.height - 180;
                            border_color = 0xbc00ff;
                            self.draw_char('M', x+10, y+10, 0xffffff);
                            self.draw_char('S', x+20, y+10, 0xffffff);
                            self.draw_char('G', x+30, y+10, 0xffffff);
                        }
                        1012 => { // InputField
                            x = 20; y = self.height - 80; w = self.width - 180; h = 50;
                            border_color = 0x00f0ff; draw_notches = false;
                            // Show last key pressed as a test of "living" input
                            if let Some(&last) = self.input_buffer.last() {
                                self.draw_char(last, x+10, y+20, 0xffffff);
                            }
                        }
                        1030 => { // SendButton
                            x = self.width - 140; y = self.height - 80; w = 120; h = 50;
                            color = 0x00f0ff; border_color = 0xffffff;
                            self.draw_char('S', x+40, y+20, 0x000000);
                            self.draw_char('E', x+50, y+20, 0x000000);
                            self.draw_char('N', x+60, y+20, 0x000000);
                            self.draw_char('D', x+70, y+20, 0x000000);
                        }
                        1234 => { // ParticleField
                            if self.particles.len() < 100 {
                                self.particles.push(Particle {
                                    x: (self.width / 2) as f32,
                                    y: (self.height / 2) as f32,
                                    vx: (self.particles.len() as f32).sin() * 2.0,
                                    vy: (self.particles.len() as f32).cos() * 2.0,
                                    life: 1.0,
                                });
                            }
                        }
                        _ => { x = 10; y = 10; w = 50; h = 50; }
                    }

                    self.update_particles();

                    for row in y..(y+h).min(self.height) {
                        for col in x..(x+w).min(self.width) {
                            self.buffer[row * self.width + col] = color;
                        }
                    }

                    for col in x..x+w {
                        if col < self.width {
                            self.buffer[y * self.width + col] = border_color;
                            self.buffer[(y+h-1).min(self.height-1) * self.width + col] = border_color;
                        }
                    }
                    for row in y..y+h {
                        if row < self.height {
                            self.buffer[row * self.width + x] = border_color;
                            self.buffer[row * self.width + (x+w-1).min(self.width-1)] = border_color;
                        }
                    }

                    if draw_notches {
                        for i in 0..10 {
                            for j in 0..(10-i) {
                                self.buffer[(y+i) * self.width + (x+j)] = 0x050506;
                                self.buffer[(y+h-1-i) * self.width + (x+w-1-j)] = 0x050506;
                            }
                        }
                    }
                }
                0x13 => { // DrawText
                    let color = self.stack.pop().unwrap_or(0.0).to_bits();
                    let y = self.stack.pop().unwrap_or(0.0) as usize;
                    let x = self.stack.pop().unwrap_or(0.0) as usize;
                    let char_code = self.stack.pop().unwrap_or(65.0) as u32;
                    self.draw_char(char_code as u8 as char, x, y, color);
                }
                0x14 => { // InputGet
                    if let Some(ref mut win) = self.window {
                        let keys = win.get_keys_pressed(minifb::KeyRepeat::No);
                        for key in keys {
                            let c = match key {
                                Key::A => 'A', Key::B => 'B', Key::C => 'C', Key::D => 'D',
                                Key::E => 'E', Key::F => 'F', Key::G => 'G', Key::H => 'H',
                                Key::I => 'I', Key::J => 'J', Key::K => 'K', Key::L => 'L',
                                Key::M => 'M', Key::N => 'N', Key::O => 'O', Key::P => 'P',
                                Key::Q => 'Q', Key::R => 'R', Key::S => 'S', Key::T => 'T',
                                Key::U => 'U', Key::V => 'V', Key::W => 'W', Key::X => 'X',
                                Key::Y => 'Y', Key::Z => 'Z', Key::Space => ' ',
                                _ => '?',
                            };
                            self.input_buffer.push(c);
                        }
                    }
                    let last_char = self.input_buffer.last().map(|&c| c as u32).unwrap_or(0);
                    self.stack.push(last_char as f32);
                }
                0x08 => { // UIRender (Flush Frame)
                    if let Some(ref mut win) = self.window {
                        win.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
                    } else {
                        println!("[UDS] Frame Synced (Virtual Framebuffer)");
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    
                    // FFmpeg Piping for Screencast
                    if self.is_recording {
                        if let Some(ref mut child) = self.ffmpeg_child {
                            if let Some(stdin) = child.stdin.as_mut() {
                                use std::io::Write;
                                let bytes: &[u8] = unsafe {
                                    std::slice::from_raw_parts(
                                        self.buffer.as_ptr() as *const u8,
                                        self.buffer.len() * 4
                                    )
                                };
                                let _ = stdin.write_all(bytes);
                            }
                        }
                    }
                }
                0x15 => { // ScreenCap
                    self.save_bmp("jarvis_screenshot.bmp");
                }
                0x16 => { // StreamCap
                    if self.is_recording {
                        self.is_recording = false;
                        if let Some(mut child) = self.ffmpeg_child.take() {
                            let _ = child.kill();
                        }
                        println!("[UDS] Screencast Stopped.");
                    } else {
                        use std::process::{Command, Stdio};
                        let child = Command::new("ffmpeg")
                            .args(&[
                                "-y", "-f", "rawvideo", "-pix_fmt", "bgra",
                                "-s", &format!("{}x{}", self.width, self.height),
                                "-r", "30", "-i", "-", "-c:v", "libx264",
                                "-pix_fmt", "yuv420p", "jarvis_screencast.mp4"
                            ])
                            .stdin(Stdio::piped())
                            .spawn();
                        
                        match child {
                            Ok(c) => {
                                self.ffmpeg_child = Some(c);
                                self.is_recording = true;
                                println!("[UDS] Screencast Started -> jarvis_screencast.mp4");
                            }
                            Err(e) => println!("[UDS] Failed to start FFmpeg: {}", e),
                        }
                    }
                }
                0x0F => { // WinUpdate (Stay open until close)
                    if let Some(ref mut win) = self.window {
                        while win.is_open() && !win.is_key_down(Key::Escape) {
                            win.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
                        }
                    }
                }
                0x12 => { // WinPoll (Non-blocking)
                    if let Some(ref mut win) = self.window {
                        win.update();
                    }
                }
                0x10 => { // EventGet
                    let last_key = if let Some(ref mut win) = self.window {
                        win.get_keys().first().cloned()
                    } else { None };
                    self.stack.push(last_key.map(|k| k as u32 as f32).unwrap_or(0.0));
                }
                0x17 => { // AsmBlock
                    println!("[JUR] [DEBUG] Entering Native Assembly Block (Simulation Mode)");
                }
                0x18 => { // VolatileWrite
                    let val = self.stack.pop().unwrap_or(0.0);
                    let addr = self.stack.pop().unwrap_or(0.0);
                    println!("[JUR] [MMIO] Volatile Write: [0x{:x}] = {}", addr as u32, val);
                }
                0x19 => { // VolatileRead
                    let addr = self.stack.pop().unwrap_or(0.0);
                    println!("[JUR] [MMIO] Volatile Read: [0x{:x}]", addr as u32);
                    self.stack.push(42.0); // Simulated hardware response
                }
                0x1A => { // AtomicOp
                    println!("[JUR] [DEBUG] Atomic Operation Executed");
                }
                0x1B => {
                    let d = self.stack.pop().unwrap_or(1.0);
                    self.is_hologram = true;
                    self.parallax_offset = (d * 5.0, d * 2.0);
                    self.alpha_modifier = 0.6;
                    println!("[JUR] [FX] Hologram Phase Shift Enabled (Depth: {})", d);
                }
                0x1C => {
                    self.is_hologram = false;
                    self.alpha_modifier = 1.0;
                    println!("[JUR] [FX] Hologram Phase Shift Disabled");
                }
                0x1D => {
                    let i = self.stack.pop().unwrap_or(0.0);
                    println!("[JUR] [FX] Applying Cyber-Glitch Shader (Intensity: {})", i);
                    self.apply_glitch_fx(i);
                }
                0x1E => {
                    let l = self.stack.pop().unwrap_or(0.0);
                    if l > 0.8 {
                        println!("[JUR] [NEURO] Focus Mode Active (Load: {}).", l);
                    }
                }
                0x00 => { // Halt
                    if self.window.is_none() && !self.buffer.is_empty() {
                        let active_pixels = self.buffer.iter().filter(|&&p| p != 0x050506).count();
                        println!("[UDS] Final Frame Summary: {} active pixels on Void Black background.", active_pixels);
                    }
                    break;
                }
                _ => {}
            }
            ip += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("--- JARVIS UNIVERSAL RUNTIME (JUR) v5.0 ---");
    let mut jur = JUR::new();
    
    if args.len() > 1 {
        let bytecode = std::fs::read(&args[1]).expect("Failed to read bytecode file");
        jur.run(&bytecode);
    } else {
        // Default "Hello GUI" Bytecode + System Ops Verification
        let bytecode: Vec<u8> = vec![
            0x01, 0x00, 0x00, 0x48, 0x44, // LoadImm 800.0
            0x01, 0x00, 0x00, 0x16, 0x44, // LoadImm 600.0
            0x0E,                         // WinCreate
            0x01, 0x00, 0x00, 0x48, 0x42, // LoadImm 50.0 (x)
            0x01, 0x00, 0x00, 0x48, 0x42, // LoadImm 50.0 (y)
            0x01, 0x00, 0x00, 0xC8, 0x42, // LoadImm 100.0 (w)
            0x01, 0x00, 0x00, 0xC8, 0x42, // LoadImm 100.0 (h)
            0x01, 0x00, 0xf0, 0xff, 0x00, // LoadImm 0x00f0ff (Neon Cyan bit pattern)
            0x11,                         // DrawRect
            
            // System Ops Verification
            0x17,                         // AsmBlock
            0x01, 0x00, 0x00, 0x00, 0x00, // LoadImm 0.0 (addr)
            0x01, 0x00, 0x00, 0x20, 0x41, // LoadImm 10.0 (val)
            0x18,                         // VolatileWrite
            0x01, 0x00, 0x00, 0x80, 0x42, // LoadImm 64.0 (addr)
            0x19,                         // VolatileRead
            0x1A,                         // AtomicOp
            
            0x00,                         // Halt
        ];
        jur.run(&bytecode);
    }
    
    println!("Execution Complete. Substrate Halted.");
}
