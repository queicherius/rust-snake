extern crate rand;
extern crate rustbox;
use rand::Rng;
use rustbox::Color;
use rustbox::Key;
use rustbox::RustBox;
use std::default::Default;
use std::error::Error;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct Screen {
    pub width: i16,
    pub height: i16,
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

fn main() {
    let mut rng = rand::thread_rng();
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut is_game_over: bool = false;

    let screen = Screen {
        width: (rustbox.width() - 2) as i16,
        height: (rustbox.height() - 1) as i16,
    };

    let mut snake_tail = Vec::new();
    snake_tail.push(Point { x: 10, y: 8 });
    snake_tail.push(Point { x: 10, y: 9 });
    snake_tail.push(Point { x: 10, y: 10 });
    let mut snake_head = Point { x: 10, y: 10 };

    let mut apple = Point {
        x: rng.gen_range(1, screen.width - 1),
        y: rng.gen_range(1, screen.height - 1),
    };

    let mut direction = Point { x: 0, y: 1 };

    loop {
        rustbox.clear();

        print_border(&rustbox, screen.width as usize, screen.height as usize);

        let frame_duration = 1000 / std::cmp::max(std::cmp::min(snake_tail.len(), 20), 10) as u64;
        match rustbox.peek_event(Duration::from_millis(frame_duration), false) {
            Ok(rustbox::Event::KeyEvent(key)) => match key {
                Key::Char('w') => direction = Point { x: 0, y: -1 },
                Key::Char('s') => direction = Point { x: 0, y: 1 },
                Key::Char('a') => direction = Point { x: -1, y: 0 },
                Key::Char('d') => direction = Point { x: 1, y: 0 },
                Key::Char('q') => break,
                _ => {}
            },
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }

        if snake_head.x > 0 && snake_head.x < screen.width {
            snake_head.x += direction.x;
        }

        if snake_head.y > 0 && snake_head.y < screen.height {
            snake_head.y += direction.y;
        }

        snake_tail.push(snake_head);

        if is_game_over
            || snake_head.x == 0
            || snake_head.y == 0
            || snake_head.x == screen.width
            || snake_head.y == screen.height
            || collides_with_itself(&snake_tail)
        {
            game_over(&rustbox, snake_tail.len() - 4);
            is_game_over = true;
            snake_tail.remove(0);
            continue;
        }

        rustbox.print(
            apple.x as usize,
            apple.y as usize,
            rustbox::RB_BOLD,
            Color::Red,
            Color::Black,
            "¤",
        );

        for (i, tail_point) in snake_tail.iter().enumerate() {
            rustbox.print(
                tail_point.x as usize,
                tail_point.y as usize,
                rustbox::RB_BOLD,
                Color::White,
                Color::Black,
                if i == snake_tail.len() - 1 { "@" } else { "0" },
            );
        }

        if snake_head.x != apple.x || snake_head.y != apple.y {
            snake_tail.remove(0);
        } else {
            // TODO This can generate inside of the tail lol

            apple = Point {
                x: rng.gen_range(1, screen.width - 1),
                y: rng.gen_range(1, screen.height - 1),
            };
        }

        rustbox.set_cursor((screen.width + 1) as isize, (screen.height) as isize);
        rustbox.present();
    }
}

fn collides_with_itself(points: &Vec<Point>) -> bool {
    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().enumerate() {
            if i != j && a.x == b.x && a.y == b.y {
                return true;
            }
        }
    }

    return false;
}

fn game_over(rustbox: &RustBox, score: usize) {
    rustbox.clear();

    rustbox.print(
        10,
        10,
        rustbox::RB_BOLD,
        Color::Red,
        Color::Black,
        "Game Over",
    );

    rustbox.print(
        10,
        11,
        rustbox::RB_BOLD,
        Color::Red,
        Color::Black,
        &format!("Score: {}", score),
    );

    rustbox.print(
        10,
        13,
        rustbox::RB_BOLD,
        Color::Red,
        Color::Black,
        "Press q to quit",
    );

    rustbox.present();
}

fn print_border(rustbox: &RustBox, width: usize, height: usize) {
    let line = "-".repeat(width);

    // Top and bottom borders
    rustbox.print(0, 0, rustbox::RB_BOLD, Color::Blue, Color::Black, &line);
    rustbox.print(
        0,
        height,
        rustbox::RB_BOLD,
        Color::Blue,
        Color::Black,
        &line,
    );

    // Left and right borders
    for y in 1..height {
        rustbox.print(0, y, rustbox::RB_BOLD, Color::Blue, Color::Black, "|");
        rustbox.print(width, y, rustbox::RB_BOLD, Color::Blue, Color::Black, "|");
    }

    // The edges
    rustbox.print(0, 0, rustbox::RB_BOLD, Color::Blue, Color::Black, "+");
    rustbox.print(0, height, rustbox::RB_BOLD, Color::Blue, Color::Black, "+");
    rustbox.print(
        width,
        height,
        rustbox::RB_BOLD,
        Color::Blue,
        Color::Black,
        "+",
    );
    rustbox.print(width, 0, rustbox::RB_BOLD, Color::Blue, Color::Black, "+");
}
