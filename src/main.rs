extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

static G_BORDER: &'static str = "*";
static G_EMPTY: &'static str = " ";
static G_APPLE: &'static str = "@";
static G_SNAKE: &'static str = "#";

#[derive(PartialEq, Debug, Clone, Copy)]
enum MapObjects {
    Empty = 0,
    Border = 1,
    Apple = 2,
    Snake = 3,
    MapObjectsCount,
}

#[derive(Debug, Copy, Clone)]
struct Object {
    x: u32,
    y: u32,
    objectType: MapObjects,
    changed: bool
}
impl Object {
    fn new() -> Object {
        Object {x: 0, y: 0, objectType: MapObjects::Empty, changed: true}
    }
}

fn main() {
    let mut map = [[Object::new(); 60]; 40];

    initMap(&mut map);

    drawMap(&map);

    inputHandle();
}

fn initMap(map: &mut [[Object; 60]; 40]) {
    for i in 0..40 {
        for j in 0..60 {
            if (i == 0 || i == 39) || (j == 0 || j == 59) {
                map[i][j].objectType = MapObjects::Border;
                map[i][j].x = j as u32;
                map[i][j].y = i as u32;
                map[i][j].changed = true;
            } else {
                map[i][j].objectType = MapObjects::Empty;
                map[i][j].x = j as u32;
                map[i][j].y = i as u32;
                map[i][j].changed = true;
            }
        }
    }
}

fn drawObject(object: &Object) {
    if true == object.changed {
        print!("{}", termion::cursor::Goto((object.x + 1) as u16, (object.y + 1) as u16));
        match object.objectType {
            MapObjects::Empty => print!("{}", G_EMPTY),
            MapObjects::Border => print!("{}", G_BORDER),
            MapObjects::Apple => print!("{}", G_APPLE),
            MapObjects::Snake => print!("{}", G_SNAKE),
            _ => print!("")
        }

    }
}

fn drawMap(map: &[[Object; 60]; 40]) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            drawObject(col);
        }
        print!("\n");
    }
}

fn inputHandle() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

