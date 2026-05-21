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
    InputGet = 0x14,
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

pub struct JUR {
    stack: Vec<f32>,
    window: Option<Window>,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    layout_y: usize,
    layout_x: usize,
    input_buffer: Vec<char>,
    is_recording: bool,
    ffmpeg_child: Option<std::process::Child>,
}

impl JUR {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            window: None,
            buffer: Vec::new(),
            width: 0,
            height: 0,
            layout_y: 0,
            layout_x: 0,
            input_buffer: Vec::new(),
            is_recording: false,
            ffmpeg_child: None,
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
                    let px = x + col;
                    let py = y + row;
                    if px < self.width && py < self.height {
                        self.buffer[py * self.width + px] = color;
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
                    let arg = self.stack.pop().unwrap_or(0.0);
                    
                    let mut x = 0; let mut y = 0; let mut w = 0; let mut h = 0;
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
                        _ => { x = 10; y = 10; w = 50; h = 50; }
                    }

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
                            if let Some(mut stdin) = child.stdin.as_mut() {
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
        // Default "Hello GUI" Bytecode
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
            0x0F,                         // WinUpdate
            0x00,                         // Halt
        ];
        jur.run(&bytecode);
    }
    
    println!("Execution Complete. Substrate Halted.");
}
