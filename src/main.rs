/// Conway's Game of Life
/// Rust Version
/// Brandon Hernandez
/// 4/17/2025
///
/// This app was originally written in Java.
/// I've been learning Rust as quick as I can, and it is the time now
/// to build something. This is a good exercise.
/// Wish me luck!
/// 

/// Functions to define:
/// [x] CellBirthOrDeath(Scanner scanner, boolean[][] map) --> String {}
/// [x] Play(boolean[][] map, int maxGens, int refreshRate) --> boolean[][] {}
/// [x] ClearCLI() {}
/// [x] Wait(int millis) {}
/// [x] PrintHeader() {}
/// [x] PrintMap(boolean[][] map, boolean brackets, boolean headers) {}
/// [x] PrintMessage(String message) {}
/// [ ] Bye() {}
/// [x] SaveMap(String filename, boolean[][] map) {}
/// [x] LoadMap(String filename) --> boolean[][] {}
/// [ ] Dir() - I need this method to print the contents of the folder to know what maps can be loaded.

/// Crates I'll need:
/// [ ] Command Line Argument Parser, to get map dimensions.
/// [x] Filesystem functions, to Load and Save maps.

struct Game {
    max_generations: u32,
    refresh_rate: u32,
    map_size: (u32, u32),
}

enum MainMenuOpt {
    CreateKillCell,
    SetGenerations,
    SetRefreshRate,
    Play,
    SaveMap,
    LoadMap,
    NewMap,
    Exit,
    // Credits,
    Unknown,
}

fn main_menu() -> MainMenuOpt {
    let menu_text: String = format!(
        "{} | {} | {} | {} | {} | {} | {} | {}\n", 
        "1. Create/kill cell", 
        "2. Set generations", 
        "3. Set refresh rate", 
        "4. Play", 
        "5. Save map", 
        "6. Load map", 
        "7. New map", 
        "99. Exit",
    );
    print_message(&menu_text, true);
    
    let opt = get_i32(&String::from("Option: "));
    
    match opt {
        1 => MainMenuOpt::CreateKillCell,
        2 => MainMenuOpt::SetGenerations,
        3 => MainMenuOpt::SetRefreshRate,
        4 => MainMenuOpt::Play,
        5 => MainMenuOpt::SaveMap,
        6 => MainMenuOpt::LoadMap,
        7 => MainMenuOpt::NewMap,
        99 => MainMenuOpt::Exit,
        _ => MainMenuOpt::Unknown, 
    }
}

type Vectrix = Vec<Vec<bool>>;
fn main() {
    clear_console();
    let mut message: String;
    message = String::from("Welcome.");
    print_message(&message, true);

    // Create a map
    let mut map: Vectrix;
    (map, message) = new_map();

    // Create game properties struct
    let mut max_generations: u32 = 0;
    let mut refresh_rate: u32 = 100;
    
    // Menu loop

    loop {
        clear_console();
        print_header();
        print_map(&map, true, true);
        print_message(&message, true);

        let menu_opt = main_menu();

        match menu_opt {
            MainMenuOpt::CreateKillCell => message = create_kill_cell(&mut map),
            MainMenuOpt::SetGenerations => (max_generations, message) = set_generations(),
            MainMenuOpt::SetRefreshRate => (refresh_rate, message) = set_refresh_rate(),
            MainMenuOpt::Play => message = play(&mut map, max_generations, refresh_rate),
            MainMenuOpt::SaveMap => message = save_map("map.txt", &map),
            MainMenuOpt::LoadMap => (map, message) = load_map("map.txt"),
            MainMenuOpt::NewMap => (map, message) = new_map(),
            MainMenuOpt::Exit => break,
            MainMenuOpt::Unknown => (),
        }
    }

}

fn get_usize(prompt: &String) -> usize {
    use std::num::ParseIntError;

    let mut result: Result<usize, ParseIntError>;

    loop {
        result = get_input(&prompt).trim().parse::<usize>();
        match result {
            Ok(_value) => break,
            Err(_error) => {
                print_message(&String::from("[-] Bad input. Try again."), true);
            },
        }
    }
    // usize value is guaranteed at this point
    result.unwrap()
}

fn get_i32(prompt: &String) -> i32 {
    use std::num::ParseIntError;

    let mut result: Result<i32, ParseIntError>;

    loop {
        result = get_input(&prompt).trim().parse::<i32>();
        match result {
            Ok(_value) => break,
            Err(_error) => {
                // clear_console();
                print_message(&String::from("[-] Bad input. Try again."), true);
            },
        }
    }
    // i32 value is guaranteed at this point
    result.unwrap()
}

fn get_u32(prompt: &String) -> u32 {
    use std::num::ParseIntError;

    let mut result: Result<u32, ParseIntError>;

    loop {
        result = get_input(&prompt).trim().parse::<u32>();
        match result {
            Ok(_value) => break,
            Err(_error) => {
                // clear_console();
                print_message(&String::from("[-] Bad input. Try again."), true);
            },
        }
    }
    // u32 value is guaranteed at this point
    result.unwrap()
}

fn get_input(prompt: &String) -> String {
    use std::io;
    let mut input = String::new();
    print_message(&prompt, true);
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => {
            panic!("[-] Failed to read input. Error details: {error}")
        },
    };
    input
}

fn new_map() -> (Vectrix, String) {
    print_message(&String::from("Generate your map."), true);
    
    let rows: usize = get_usize(&String::from("Rows:"));
    let cols: usize = get_usize(&String::from("Cols:"));

    if rows == 0 || cols == 0 {
        return (
            vec![vec![false; 10]; 10], 
            String::from(format!("[-] Invalid dimensions. 10 x 10 map created."))
        );
    }

    (
        vec![vec![false; cols]; rows], 
        String::from(format!("[+] {rows} x {cols} map created."))
    )
}

fn print_message(message: &String, new_line: bool) {
    // println!();
    // print!("\r");
    // print!("{message}");
    if new_line {
        println!("{message}");
    } else {
        print!("{message}");
    }
}

fn print_header() {
    println!("
    =========================================================
    Conway's Game of Life
    =========================================================
    ");
}

fn clear_console() {
    // ANSI escape code to clear screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
    // io::stdout().flush().unwrap(); // make sure it actually prints now (?)
}

fn refresh_console() {
    // ANSI code to set cursor to row 0, column 0.
    print!("\x1b[0;0H");
}

fn print_map(map: &Vectrix, brackets: bool, headers: bool) {
    let cell: String = match brackets {
        true => format!("[{} ]", "■"),
        false => format!(" {}  ", "■"),
    };
    let no_cell: String = match brackets {
        true => String::from("[  ]"),
        false => String::from("    "),
    };
    let rows: usize = map.len();
    let cols: usize = map[0].len();

    let mut header: String;

    // Column header print
    for i in 0..cols {
        // Top left corner is blank
        if i == 0 {
            print!("    ");
        }
        header = match headers {
            true => format!("[{i:>2}]"),
            false => format!("    "),
        };
        print!("{header}");
    }
    println!();

    // Map print
    for i in 0..rows {
        // Row header print
        header = match headers {
            true => format!("[{i:>2}]"),
            false => format!("    "),
        };
        print!("{header}");

        for j in 0..cols {
            print!("{}", match map[i][j] {
                true => &cell,
                false => &no_cell,
            });
        }
        println!();
    }
}

fn create_kill_cell(map: &mut Vectrix) -> String {
    
    let row: usize = get_usize(&String::from("Row: "));
    let col: usize = get_usize(&String::from("Col: "));
    
    clear_console();

    let row_len = map.len();
    let col_len = map[0].len();

    let filtered_row = row % row_len;
    let filtered_col = col % col_len;

    map[filtered_row][filtered_col] = !map[filtered_row][filtered_col];

    match map[filtered_row][filtered_col] {
        true => { 
            return format!("[■ ] Alive cell at [{filtered_row:>2}][{filtered_row:>2}]");
        },
        false => { 
            return format!("[  ] Dead cell at [{filtered_row:>2}][{filtered_col:>2}]");
        },
    }
}

fn set_generations() -> (u32, String) {
    let generations: u32 = get_u32(&String::from("Generations: ")); 
    
    clear_console();

    return (
        generations, 
        format!("Generations = {generations}")
    );
}

fn set_refresh_rate() -> (u32, String) {
    let rate: u32 = get_u32(&String::from("Refresh rate: ")); 
    return (
        rate, 
        format!("Refresh rate = {rate}")
    );
}

fn play(map: &mut Vectrix, max_generations: u32, refresh_rate: u32) -> String {
    let mut generations: u32 = 0;
    let i_size: usize = map.len();
    let j_size: usize = map[0].len();
    
    clear_console();

    loop {
        refresh_console();
        print_header();
        print_map(map, false, false);
        let message = format!("Generation {generations} of {max_generations}");
        print_message(&message, true);
        delay(refresh_rate);

        // Just needed that extra print of the last generation.
        // Break the loop now.
        if generations == max_generations {
            break;
        }
        
        let mut next_map: Vectrix = vec![vec![false; j_size]; i_size];
        
        for i in 0..i_size {
            for j in 0..j_size {
                let neighbors = calculate_neighbors(map, &i, &j);
                calculate_next_gen(&map, &mut next_map, neighbors, &i, &j);
            }
        }
        *map = next_map;
        generations += 1;
    }
    // Return message
    String::from("Game finished.")
}

fn calculate_next_gen(map: &Vectrix, next_map: &mut Vectrix, neighbors: u32, i: &usize, j: &usize) {
    
    // 1. Any live cell with fewer than 2 live neighbors dies, as if by underpopulation.
    if map[*i][*j] && neighbors < 2 {
        next_map[*i][*j] = false;
    }
    // 2. Any live cell with 2 or 3 live neighbors lives on to the next generation.
    if map[*i][*j] && (neighbors == 2 || neighbors == 3) {
        next_map[*i][*j] = true;
    }
    // 3. Any live cell with more than 3 live neighbors dies, as if by overpopulation.
    if map[*i][*j] && neighbors > 3 {
        next_map[*i][*j] = false;
    }
    // 4. Any dead cell with exactly 3 live neighbors becomes a live cell, as if by reproduction.
    if !map[*i][*j] && neighbors == 3 {
        next_map[*i][*j] = true;
    }

}

fn calculate_neighbors(map: &Vectrix, i: &usize, j: &usize) -> u32 {
    let i_last: i32 = (map.len() - 1) as i32;
    let j_last: i32 = (map[0].len() - 1) as i32;
    
    // This is needed because you cannot make usize integers go negative.
    let i: i32 = *i as i32;
    let j: i32 = *j as i32;
    
    let mut i_chk: i32;
    let mut j_chk: i32;
    let mut i_chk_ptr: usize;
    let mut j_chk_ptr: usize;
    
    let mut neighbors = 0;
    
    // [i-1,j-1]  [i-1,_j_]  [i-1,j+1]
    // [_i_,j-1]  [_i_,_j_]  [_i_,j+1]
    // [i+1,j-1]  [i+1,_j_]  [i+1,j+1]

    // [i-1,j-1] : NW
    // [i-1,_j_] : N
    // [i-1,j+1] : NE

    // [_i_,j-1] : W
    // [_i_,j+1] : E

    // [i+1,j-1] : SW
    // [i+1,_j_] : S
    // [i+1,j+1] : SE

    // [i-1,j-1] : NW
    i_chk = i - 1;
    j_chk = j - 1;
    if i_chk < 0 {
        i_chk = i_last;
    }
    if j_chk < 0 {
        j_chk = j_last;
    }
    i_chk_ptr = i_chk as usize;
    j_chk_ptr = j_chk as usize;
    neighbors = match map[i_chk_ptr][j_chk_ptr] {
        true => neighbors + 1,
        false => neighbors,
    };

    // [i-1,_j_] : N
    i_chk = i - 1;
    j_chk = j;
    if i_chk < 0 {
        i_chk = i_last;
    }
    i_chk_ptr = i_chk as usize;
    j_chk_ptr = j_chk as usize;
    neighbors = match map[i_chk_ptr][j_chk_ptr] {
        true => neighbors + 1,
        false => neighbors,
    };

    // [i-1,j+1] : NE
    i_chk = i - 1;
    j_chk = j + 1;
    if i_chk < 0 {
        i_chk = i_last;
    }
    if j_chk > j_last {
        j_chk = 0;
    }
    i_chk_ptr = i_chk as usize;
    j_chk_ptr = j_chk as usize;
    neighbors = match map[i_chk_ptr][j_chk_ptr] {
        true => neighbors + 1,
        false => neighbors,
    };

    // [_i_,j-1] : W
    i_chk = i;
    j_chk = j - 1;
    if j_chk < 0 {
        j_chk = j_last;
    }
    i_chk_ptr = i_chk as usize;
    j_chk_ptr = j_chk as usize;
    neighbors = match map[i_chk_ptr][j_chk_ptr] {
        true => neighbors + 1,
        false => neighbors,
    };

    // [_i_,j+1] : E
    i_chk = i;
    j_chk = j + 1;
    if j_chk > j_last {
        j_chk = 0;
    }
    i_chk_ptr = i_chk as usize;
    j_chk_ptr = j_chk as usize;
    neighbors = match map[i_chk_ptr][j_chk_ptr] {
        true => neighbors + 1,
        false => neighbors,
    };

    // [i+1,j-1] : SW
    i_chk = i + 1;
    j_chk = j - 1;
    if i_chk > i_last {
        i_chk = 0;
    }
    if j_chk < 0 {
        j_chk = j_last;
    }
    i_chk_ptr = i_chk as usize;
    j_chk_ptr = j_chk as usize;
    neighbors = match map[i_chk_ptr][j_chk_ptr] {
        true => neighbors + 1,
        false => neighbors,
    };

    // [i+1,_j_] : S
    i_chk = i + 1;
    j_chk = j as i32;
    if i_chk > i_last {
        i_chk = 0;
    }
    i_chk_ptr = i_chk as usize;
    j_chk_ptr = j_chk as usize;
    neighbors = match map[i_chk_ptr][j_chk_ptr] {
        true => neighbors + 1,
        false => neighbors,
    };

    // [i+1,j+1] : SE
    i_chk = i + 1;
    j_chk = j + 1;
    if i_chk > i_last {
        i_chk = 0;
    }
    if j_chk > j_last {
        j_chk = 0;
    }
    i_chk_ptr = i_chk as usize;
    j_chk_ptr = j_chk as usize;
    neighbors = match map[i_chk_ptr][j_chk_ptr] {
        true => neighbors + 1,
        false => neighbors,
    };

    neighbors
}

fn delay(millis: u32) {
    std::thread::sleep(std::time::Duration::from_millis(millis as u64));
}

fn load_map(filename: &str) -> (Vectrix, String) {
    use std::fs::read_to_string;
    
    // Return default map of 2x2 if failed to read file.
    let content = match read_to_string(filename) {
        Ok(content) => content,
        Err(_) => return (vec![vec![false; 2]; 2], format!("[-] Failed to load map.")),
    };
    
    let content = content.chars();
    
    // A unit is [x] or [ ]
    let mut unit: String = String::new();
    
    // Init map with one row
    let mut map: Vectrix = vec![vec![]; 1];
    
    // Start pushing values on first row
    let mut i: usize = 0;
    
    for char in content {
        unit.push(char);
        
        // If unit is an alive cell, push True
        if unit.eq("[x]") {
            map[i].push(true);
            unit.clear();
        };

        // If unit is a dead cell, push False
        if unit.eq("[ ]") {
            map[i].push(false);
            unit.clear();
        };

        // If new line, push Row
        if unit.eq("\r\n") {
            map.push(vec![]);
            i += 1;
            unit.clear();
        }
    };

    (map, format!("[+] Map was loaded."))
}

fn save_map(filename: &str, map: &Vectrix, ) -> String {
    use std::fs::write;

    let mut content = String::new();

    // The peekable() method in the iterator allows
    // us to look ahead into the collection.
    // I decided to take this approach so I could look
    // into the next row of the map, and if there is none,
    // then I wouldn't append a new line.

    let mut iter = map.iter().peekable();

    while let Some(row) = iter.next() {
        for cell in row {
            let str_to_push = match cell {
                true => "[x]",
                false => "[ ]",
            };
            content.push_str(str_to_push);                
        }
        // If there is a another row next...
        if iter.peek().is_some() {
            // Windows newline sequence is CR LF (Carriage Return Line Feed)
            content.push_str("\r\n");
        }
    }

    match write(filename, content.into_bytes()) {
        Ok(_) => return format!("[+] Map saved."),
        Err(_) => return format!("[-] Failed to save map."),
    };
}