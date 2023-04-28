use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    Result, execute
};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType};
use std::{io::stdout};
use terminal_size::{Width, Height, terminal_size};

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
// Definer en funksjon kalt `makeline` som tar fire argumenter av typen `i32`.
pub fn makeline(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {

    // Beregn stigningen til linjen.
    let m: i32 = 2 * (y2 - y1);

    // Initialiser stigningsfeilen med verdien `m - (x2 - x1)`.
    let mut slope_error: i32 = m - (x2 - x1);

    // Initialiser en tom vektor som vil inneholde koordinatene til linjen.
    let mut vec = Vec::new();

    // Initialiser en variabel `y` med verdien `y1`.
    let mut y = y1;

    // Løkke gjennom verdiene av `x` fra `x1` til `x2`, inkludert.
    for x in x1..=x2 {

        // Legg til et tupel `(x, y)` som representerer et punkt på linjen, til `vec` vektoren.
        vec.push((x, y));

        // Oppdater verdien av `slope_error` ved å legge til `m`.
        slope_error = slope_error + m;

        // Hvis `slope_error` er større enn eller lik `0`, øk verdien av `y` med 1 og trekk fra `2 * (x2 - x1)` fra `slope_error`.
        if slope_error >= 0 {
            y = y+1;
            slope_error = slope_error - 2 * (x2 - x1)
        }
    }

    // Returner `vec` vektoren som inneholder koordinatene til linjen.
    return vec;
}

fn main() {
    // Skru på alternativ skjermmodus for å kunne skrive ut tegn på spesifikke koordinater
    execute!(stdout(), EnterAlternateScreen).expect("Kunne ikke skru på alternativ skjermmodus.");

    let height: u16;

    let size = terminal_size();
    if let Some((Width(_w), Height(h))) = size { height = h; } else { height = 10; }

    let mut num = 0;
    while num == 0 {

        // Flytt markøren til en bestemt plassering og les inn et linjeskift
        movecursorn(1, height);
        let input_str = read_line().unwrap();
        execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();


        // Sjekk om brukeren vill lukke programmet
        if input_str == "q" {
            num = 1
        }else {
            // hvis ikke split resultatet inn i deler og putt det in i variabler
            let mut coords = input_str.split(|c| c == ',' || c == ':')
            .map(|s| s.parse::<i32>().unwrap());
    
            let (x1, y1, x2, y2) = (coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap());
            
            // Lag en liste med koordinatene til linjen
            let cordlist: Vec<(i32, i32)> = makeline(x1, y1, x2, y2);

            // Gå gjennom hvert koordinat i linjen og skriv ut et "X" på den spesifikke plasseringen
            for i in 0..=cordlist.len()-1 {
                let cordtuple = cordlist[i];
                printat(cordtuple.0.try_into().unwrap(), cordtuple.1.try_into().unwrap(), "X");
            }
        }
    }
    // Skru av alternativ skjermmodus
    execute!(stdout(), LeaveAlternateScreen).expect("Kunne ikke skru av alternativ skjermmodus.");
}