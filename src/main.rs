use std::io::{self, stdin, stdout};
use crossterm::event::{poll, read, Event};
use cgmath::{Vector2, dot, ElementWise};
use std::thread::sleep;
use std::time;
#[derive(Clone,Debug, Copy)]
enum Pixel {
    Empty,
    Full
}
type V2ii = Vector2<i32>;
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

    fn show(&self){
        println!("+--------------------------------------------------------------------------------+");
        let mut i = 0;
        for _ in 0..(HEIGHT/2) {
            let s = String::from_iter(&self.b[i..i+WIDTH]);
            print!("|");
            print!("{}", s);
            println!("|");
            i += WIDTH
        }
        println!("+--------------------------------------------------------------------------------+");
    }
    fn circle(&mut self, pos: V2ii, r: i32){
        let x1 = pos.x;
        let y1 = pos.y;
        
        let rr = V2ii::new(r, r);
        let bottom_left_corner = pos - rr;
        let top_right_corner = pos + rr;
        
        // for y in 0..HEIGHT as i32 {
        //     for x in 0..WIDTH as i32 {
        for y in bottom_left_corner.y..=top_right_corner.y {
            for x in bottom_left_corner.x..=top_right_corner.x {
                // if ((x as i32 - x1)^2 + (y as i32 - y1)^2) as usize <= r^2 {
                if (0 <= x) && (x < WIDTH as i32) && (0 <= y) && (y < HEIGHT as i32) {
                    let dx = pos.x - x; 
                    let dy = pos.y - y;
                    if ((dx)*(dx) + (dy)*(dy)) <= r*r {
                        self.a[y as usize * WIDTH + x as usize] = Pixel::Full;
                    }
                }
            }
        }
        // println!("{:?}", self.a);
        for y in 0..(HEIGHT/2) as i32 {
            for x in 0..WIDTH as i32 {
                self.b[y as usize * WIDTH + x as usize] = match (self.a[(2 * y) as usize * WIDTH + x as usize], self.a[(2 * y + 1) as usize * WIDTH + x as usize]){
                        (Pixel::Empty, Pixel::Empty) => ' ',
                        (Pixel::Full, Pixel::Empty) => '^',
                        (Pixel::Empty, Pixel::Full) => '_',
                        (Pixel::Full, Pixel::Full) => '0',
                    };

                // if ((x_-x1)*(x_-x1) + (y_-y1)*(y_-y1)) as i32 <= r*r {
                //     // dis.a[y * HEIGHT + x] = Pixel::Full;
                //     self.b[y * WIDTH + x] = '*';
                // }
                // if(x == 0 || x == WIDTH - 1){
                //     self.b[y * WIDTH + x] = '|';
                // }
                // if(y == 0 || y == HEIGHT - 1){
                //     self.b[y * WIDTH + x] = '-';
                // }
                // if(x == 0 && y == 0){
                //     self.b[y * WIDTH + x] = '+';
                // }
                // if(x == 0 && y == HEIGHT - 1){
                //     self.b[y * WIDTH + x] = '+';
                // }
                // if(x == WIDTH - 1 && y == 0){
                //     self.b[y * WIDTH + x] = '+';
                // }
                // if(x == WIDTH - 1  && y == HEIGHT - 1){
                //     self.b[y * WIDTH + x] = '+';
                // }
                 
            }
        }
    }
}

fn main() {
    let mut D = ScreenBuffer::init();
    // let game = true;
    let one_micro = time::Duration::from_micros(1);
    let fps = one_micro * 1000 * 1000 / 30;
    let mut pos = V2ii::new(0, 0);
    let mut r = 6;
    let mut acceleration = -100;
    let mut velocity = 10;
    let c = V2ii::new(0, HEIGHT as i32 - 1);
    let d = V2ii::new(-1, 1);
    println!("{:?}", -c + pos);
    let mut i = 0;
    let turns = 4;
    loop {
        if i > turns {
            break;
        }
        D.circle((c - pos).mul_element_wise(d) , r);
        // D.circle(p, r);
        D.show();
        
        std::thread::sleep(fps);
        i+= 1;
    }

}
