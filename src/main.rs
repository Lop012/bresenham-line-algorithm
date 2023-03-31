use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    Result, execute
};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use std::{io::stdout};


pub fn read_line() -> Result<String> {
    let mut line = String::new();
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
  
          KeyCode::Enter => {
              break;
          }
  
          KeyCode::Char(c) => {
              line.push(c);
          }
          _ => {}
        }
    }
  
    return Ok(line);
}
  
  
  
  pub fn movecursorn(x: u16, y: u16) {
    execute!(stdout(),crossterm::cursor::MoveTo(x, y)).expect("Error Moving Cursor");
  }

  pub fn printat(x: u16, y: u16, msg: &str) {
    execute!(stdout(),crossterm::cursor::MoveTo(x, y)).expect("msg");
    print!("{}", msg)
  }

// Bresenham's Line Algorithm
pub fn makeline(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {

    let m: i32 = 2 * (y2 - y1);
    let mut slope_error: i32 = m - (x2 - x1);

    let mut vec = Vec::new();

    let mut y = y1;

    for x in x1..=x2 {

        // Vi putter dem inn i en "Vector" (Egentlig bare en liste)
        // Sånn at vi kan senere bruke dem
        vec.push((x, y));

        slope_error = slope_error + m;


        // hvis slope error her på sin "limit"
        // Må vi plusse en til y
        // også kalkulere en ny slope error
        if slope_error >= 0 {
            y = y+1;
            slope_error = slope_error - 2 * (x2 - x1)
        }
    }
    return vec;
}


fn main() {

    let x1: i32 = 1;
    let y1: i32 = 1;
    let x2: i32 = 250;
    let y2: i32 = 7;

    let mut num = 0;
    // Runner funksjonen
    let cordlist: Vec<(i32, i32)> = makeline(x1, y1, x2, y2);
    execute!(stdout(), EnterAlternateScreen).expect("msg");
    while num == 0 {
        for i in 0..=cordlist.len()-1 {
            let cordtuple = cordlist[i];
            printat(cordtuple.0.try_into().unwrap(), cordtuple.1.try_into().unwrap(), "X")
        }
        movecursorn(1, 10);

        // Når brukeren skriver inn noe og trykker enter, gå ut av loopen
        // kan brukes senere for å få input fra brukeren og lage en linje ut av det brukeren skriver
        let _str = read_line();
        num = 1

    }
    // Gå ut av alternativ skjerm
    execute!(stdout(), LeaveAlternateScreen).expect("msg");
}