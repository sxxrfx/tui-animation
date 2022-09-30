use std::io::{self, Write, stdout};
use crossterm::event::{poll, read, Event};
use crossterm::{
    execute, queue, Result, ExecutableCommand,
    terminal::{ScrollUp, SetSize, size},
    style::{self, Stylize, Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute},
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition}
};
use cgmath::{Vector2, dot, ElementWise};
use std::thread::sleep;
use std::time;
#[derive(Clone,Debug, Copy)]
enum Pixel {
    Empty,
    Full
}
type V2ff = Vector2<f32>;


const WIDTH: usize = 80;
const HEIGHT: usize = 40;
const LEN_A: usize = WIDTH * HEIGHT;
const LEN_B: usize = LEN_A /2;
// const LEN_B: usize = LEN_A; 
#[derive(Debug)]
struct ScreenBuffer{
    a: [Pixel; LEN_A],
    b: [char; LEN_B]
}

impl ScreenBuffer {
    fn init() -> Self{
        let a = [Pixel::Empty; LEN_A];
        let b = [' '; LEN_B];
        ScreenBuffer {
            a,
            b
        }
    }
    fn refresh(&mut self){
        self.a = [Pixel::Empty; LEN_A];
        self.b = [' '; LEN_B];
    }


    fn show(&self){
        let up = format!("\u{001b}[{}A", (HEIGHT/2) + 1);
        let left = format!("\u{001b}[{}D", WIDTH + 2);
        let mut stdout = stdout();
        // let (cols, rows) = size().unwrap();
        execute!(stdout, 
            // SetSize(WIDTH as u16, (HEIGHT/2) as u16),
            // terminal::Clear(terminal::ClearType::All),
            // SavePosition,
            SetForegroundColor(Color::Blue),
            // Red background
            // SetBackgroundColor(Color::Red),
            // Print text
            // Print("Blue text on Red.".to_string()),
            // Reset to default colors
            DisableBlinking,
            style::Print("+--------------------------------------------------------------------------------+\n".to_string())
        ).unwrap();


        let mut i = 0;
        for _ in 0..(HEIGHT/2) {
            // let s = String::from_iter(&self.b[i..i+WIDTH]);
            let s = format!("|{}|\n", String::from_iter(&self.b[i..i+WIDTH]));
            queue!(stdout, 
                // style::Print("|".to_string()),
                style::Print(s),
                // style::Print("|\n".to_string()),
            ).unwrap();
            i += WIDTH
        }
        execute!(stdout, 
            style::Print("+--------------------------------------------------------------------------------+".to_string()),
            style::Print(&left),
            style::Print(&up),
            ResetColor,
            // MoveTo(0, 0),
        ).unwrap();
        // stdout.execute(RestorePosition).unwrap();
        stdout.flush().unwrap();
    }
    fn circle(&mut self, pos: V2ff, r: f32){
        let rr = V2ff::new(r, r);
        let bl = pos - rr;
        let tr = pos + rr;
        let bx = f32::floor(bl.x) as i32;
        let by = f32::floor(bl.y) as i32;
        let tx = f32::ceil(tr.x) as i32;
        let ty = f32::ceil(tr.y) as i32;
        
        for y in by..=ty {
            for x in bx..=tx {
                if (0 <= x) && (x < WIDTH as i32) && (0 <= y) && (y < HEIGHT as i32) {
                    let dx = pos.x - (x as f32 + 0.5) as f32; 
                    let dy = pos.y - (y as f32 + 0.5) as f32;
                    if ((dx)*(dx) + (dy)*(dy)) <= r*r {
                        self.a[y as usize * WIDTH + x as usize] = Pixel::Full;
                    }
                }
            }
        }
        for y in 0..(HEIGHT/2) as i32 {
            for x in 0..WIDTH as i32 {
                self.b[y as usize * WIDTH + x as usize] = match (self.a[(2 * y) as usize * WIDTH + x as usize], self.a[(2 * y + 1) as usize * WIDTH + x as usize]){
                        (Pixel::Empty, Pixel::Empty) => ' ',
                        (Pixel::Full, Pixel::Empty) => '^',
                        (Pixel::Empty, Pixel::Full) => '_',
                        (Pixel::Full, Pixel::Full) => '0',
                    };
            }
        }
    }
}

fn main() {
    let mut scrbuf = ScreenBuffer::init();
    let one_micro = time::Duration::from_micros(1);
    let fps = 30;
    let sleep_time = one_micro * 1000 * 1000 / fps;
    let r: f32 = 8.0;
    let mut pos = V2ff::new(0.0, 30.0);
    let acc =V2ff::new(0.0, -9.8) / fps as f32;

    let loss = 0.05_f32;
    let mut velocity = V2ff::new(35.0, -1.0) / fps as f32;

    const C: Vector2<f32> = V2ff::new(0.0, HEIGHT as f32 - 1.0);
    const D: Vector2<f32> = V2ff::new(-1.0, 1.0);
    let mut i = 0;
    let turns = 1000;
    loop {
        if i > turns {
            break;
        }
        if pos.x > WIDTH as f32 {
            pos = V2ff::new(0.0, 30.0);
            velocity = V2ff::new(35.0, -1.0) / fps as f32;

        }
        if velocity.y < 0.0 && pos.y <= r  {
            // if pos.y <= 0.0 {
            //     pos.y = -pos.y;
            // }
            // pos.y = 1.4 * r - pos.y;
            pos.y = r; 
            velocity.y *= -1.0 * (1.0-loss);
            // velocity.y *= -1.0;
        }
        scrbuf.circle((C - pos).mul_element_wise(D) , r);
        scrbuf.show();
        scrbuf.refresh();
        
        std::thread::sleep(sleep_time);
        i+= 1;
        velocity += acc;
        pos += velocity;
    }

}
