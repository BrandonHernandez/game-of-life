// Conway's Game of Life
// Rust Version
// Brandon Hernandez
// 4/17/2025
//
// This app was originally written in Java.
// I've been learning Rust as quick as I can, and it is the time now
// to build something. This is a good exercise.
// Wish me luck!
// 

// Functions to define:
// [x] CellBirthOrDeath(Scanner scanner, boolean[][] map) --> String {}
// [x] Play(boolean[][] map, int maxGens, int refreshRate) --> boolean[][] {}
// [x] ClearCLI() {}
// [x] Wait(int millis) {}
// [x] PrintHeader() {}
// [x] PrintMap(boolean[][] map, boolean brackets, boolean headers) {}
// [x] PrintMessage(String message) {}
// [ ] Bye() {}
// [x] SaveMap(String filename, boolean[][] map) {}
// [x] LoadMap(String filename) --> boolean[][] {}
// [ ] Dir() - I need this method to print the contents of the folder to know what maps can be loaded.

// Crates I'll need:
// [ ] Command Line Argument Parser, to get map dimensions.
// [x] Filesystem functions, to Load and Save maps.

type Vectrix = Vec<Vec<Cell>>;

fn main() {
    clear_console();
    let mut message: String;
    message = String::from("Welcome.");
    print_message(&message, true);

    // Create a map
    let mut map: Vectrix;
    (map, message) = Map::new();
    
    // Game config struct
    let mut game_properties = GameConfig {
        tick_rate: 250,
        infinite_game: true,
        max_generations: 50,
        map_size: (10, 10),
    };

    // Menu loop
    loop {
        clear_console();
        print_header(vec!["Game of Life"]);
        print_map(&map, true, true);
        print_message(&message, true);

        let menu_opt = main_menu();

        match menu_opt {
            MainMenuOpt::SetClearCell => {
                message = Map::set_clear_cell(&mut map)
            },
            MainMenuOpt::GeneratePattern => {
                // Option in development. Need to give ability of choosing pattern.
                message = String::from("Generate pattern. Set origin.");
                loop {
                    clear_console();
                    print_header(vec!["Fame of Light"]);
                    print_map(&map, true, true);
                    print_message(&message, true);

                    let (row, aborted) = get_usize(&String::from("Row"), true);
                    if aborted {
                        break;
                    }
                    
                    let (col, aborted) = get_usize(&String::from("Col"), true);
                    if aborted {
                        break;
                    }

                    let origin = Coordinates::new(row, col);

                    let glider = Spaceship::glider(origin.clone());

                    let fifteen_bent_paperclip = StillLife::fifteen_bent_paperclip(origin);

                    let glider_points = Spaceship::get_points(glider);

                    let fift_paperclip_points = StillLife::get_points(fifteen_bent_paperclip);

                    // message = Map::generate_spaceship(&mut map, glider);
                    // message = Map::generate_still_life(&mut map, fifteen_bent_paperclip);
                    message = Map::generate_pattern(&mut map, glider_points);

                }
                message = String::from("[+] Pattern generation finished.");
            },
            MainMenuOpt::Play => {
                message = play(&mut map, &game_properties);
            },
            MainMenuOpt::SaveMap => {
                message = save_map("map.txt", &map)
            },
            MainMenuOpt::LoadMap => {
                (map, message) = load_map("map.txt")
            },
            MainMenuOpt::Configuration => {
                message = String::from("Game configuration");
                // Menu loop
                loop {
                    clear_console();
                    print_header(vec!["Game of Life"]);
                    print_map(&map, true, true);
                    print_message(&message, true);

                    let menu_opt = config_menu();

                    match menu_opt {
                        ConfigMenuOpt::SetTickRate => {
                            (game_properties.tick_rate, message) = set_tick_rate();
                        },
                        ConfigMenuOpt::InfiniteGame => {
                            (game_properties.infinite_game, message) = set_infinite_game(&game_properties.infinite_game);
                        },
                        ConfigMenuOpt::SetMaxGenerations => {
                            (game_properties.max_generations, message) = set_generations();
                        },
                        ConfigMenuOpt::SetMapSize => (map, message) = Map::new(),
                        ConfigMenuOpt::Exit => break,
                        ConfigMenuOpt::Unknown => (),
                    }
                }
            },
            MainMenuOpt::Exit => break,
            MainMenuOpt::Unknown => (),
        }
    }

}

enum MainMenuOpt {
    SetClearCell,
    GeneratePattern,
    Play,
    SaveMap,
    LoadMap,
    Configuration,
    Exit,
    // Credits,
    Unknown,
}

fn main_menu() -> MainMenuOpt {
    let menu_text: String = format!(
        "{} | {} | {} | {} | {} | {} | {}\n", 
        "1. Set/Clear cell",
        "2. Generate pattern", 
        "3. Play", 
        "4. Save map", 
        "5. Load map", 
        "6. Configuration", 
        "99. Exit",
    );
    print_message(&menu_text, true);
    
    let opt = get_u32(&String::from("Option: "));
    
    match opt {
        1 => MainMenuOpt::SetClearCell,
        2 => MainMenuOpt::GeneratePattern,
        3 => MainMenuOpt::Play,
        4 => MainMenuOpt::SaveMap,
        5 => MainMenuOpt::LoadMap,
        6 => MainMenuOpt::Configuration,
        99 => MainMenuOpt::Exit,
        _ => MainMenuOpt::Unknown, 
    }
}

struct GameConfig {
    tick_rate: u32,
    infinite_game: bool,
    max_generations: u32,
    map_size: (u32, u32),
}

// This function is under development
impl GameConfig {
    fn get_config() -> GameConfig {
        use std::fs::read_to_string;

        let filename = "config.ini";
        let config = match read_to_string(filename) {
            Ok(data) => data,
            Err(_) => return GameConfig {
                tick_rate: 75,
                infinite_game: true,
                max_generations: 9999,
                map_size: (10, 10),
            },
        };
        GameConfig {
            tick_rate: 75,
            infinite_game: true,
            max_generations: 9999,
            map_size: (10, 10),
        }
    }
}

enum ConfigMenuOpt {
    SetTickRate,
    InfiniteGame,
    SetMaxGenerations,
    SetMapSize,
    Exit,
    Unknown,
}

fn config_menu() -> ConfigMenuOpt {
    let config_text: String = format!(
        "{} | {} | {} | {} | {}\n",
        "1. Set Tick Rate",
        "2. Infinite game",
        "3. Set Max Generations",
        "4. Set Map Size",
        "99. Exit",
    );
    print_message(&config_text, true);

    let opt = get_u32(&String::from("Option: "));

    match opt {
        1 => ConfigMenuOpt::SetTickRate,
        2 => ConfigMenuOpt::InfiniteGame,
        3 => ConfigMenuOpt::SetMaxGenerations,
        4 => ConfigMenuOpt::SetMapSize,
        99 => ConfigMenuOpt::Exit,
        _ => ConfigMenuOpt::Unknown,
    }
}

#[derive(Clone)]
enum Cell {
    Alive(String),
    Dead(String),
}

impl Cell {
    fn alive() -> Cell {
        Cell::Alive(String::from("■"))
    }
    fn dead() -> Cell {
        Cell::Dead(String::from(" "))
    }
    fn not(&self) -> Cell {
        match self {
            Cell::Alive(_) => Cell::dead(),
            Cell::Dead(_) => Cell::alive(),
        }
    }
}

fn get_usize(prompt: &String, abort_feature: bool) -> (usize, bool) {
    const ABORTED: bool = true;
    
    let mut prompt_mod = prompt.clone();
    if abort_feature {
        prompt_mod.push_str("\nUse `q` to quit.");
    }
    
    loop {    
        let input_str = get_input(&prompt_mod);
        let input_trim = input_str.trim();
        
        // Check result str first to match `q`. If yes, abort.
        if input_trim == "q" {
            return (0 as usize, ABORTED);
        }
    
        match input_trim.parse::<usize>() {
            Ok(value) => {
                return (value, !ABORTED);
            },
            Err(_error) => {
                print_message(&String::from("[-] Bad input. Try again."), true);
            },
        }
    }
}

fn get_u32(prompt: &String) -> u32 {
    loop {
        match get_input(prompt).trim().parse::<u32>() {
            Ok(value) => {
                return value;
            },
            Err(_error) => {
                print_message(&String::from("[-] Bad input. Try again."), true);
            },
        }
    }
}

fn get_input(prompt: &String) -> String {
    use std::io;
    let mut input = String::new();
    print_message(&prompt, true);
    match io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            return input;
        },
        Err(error) => {
            panic!("[-] Failed to read input. Error details: {error}")
        },
    };
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

fn print_header(header_contents: Vec<&str>) {
    let mut decor = String::new();
    for _ in 0..70 {
        decor.push('=');
    }
    println!("{decor}");
    for piece in header_contents {
        print!("{} ", piece);
    }
    print!("\n");
    println!("{decor}");
}

fn clear_console() {
    // ANSI escape code to clear screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
}

fn refresh_console() {
    // ANSI code to set cursor to row 0, column 0.
    print!("\x1b[0;0H");
}

fn print_map(map: &Vectrix, brackets: bool, headers: bool) {
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
            let ch = match &map[i][j] {
                Cell::Alive(ch) => ch,
                Cell::Dead(ch) => ch,
            };
            if brackets {
                print!("[{ch} ]");
            }
            if !brackets {
                print!(" {ch}  ");
            }
        }
        println!();
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

fn set_tick_rate() -> (u32, String) {
    let rate: u32 = get_u32(&String::from("Tick rate (ms): ")); 
    return (
        rate, 
        format!("Tick rate = {rate} ms")
    );
}

fn play(map: &mut Vectrix, game_properties: &GameConfig) -> String {
    let mut generations: u32 = 0;
    let i_size: usize = map.len();
    let j_size: usize = map[0].len();
    
    clear_console();

    loop {
        refresh_console();
        print_header(vec!["Game of Life"]);
        print_map(map, false, false);
        let message = match game_properties.infinite_game {
            true => format!("Generation {}", generations),
            false => format!("Generation {} of {}", generations, game_properties.max_generations),
        };
        
        print_message(&message, true);
        delay(game_properties.tick_rate);

        // Just needed that extra print of the last generation.
        // Break the loop now.
        if generations == game_properties.max_generations && !game_properties.infinite_game {
            // Return message
            return String::from("Game finished.");
        }

        // Return if ESC is pressed
        if esc_key_pressed() {
            return String::from("Game aborted.");
        }
        
        let mut next_map: Vectrix = vec![vec![Cell::dead(); j_size]; i_size];
        
        for i in 0..i_size {
            for j in 0..j_size {
                let neighbors = calculate_neighbors(map, &i, &j);
                calculate_next_gen(&map, &mut next_map, neighbors, &i, &j);
            }
        }
        *map = next_map;
        generations += 1;
    }
    
}

fn calculate_next_gen(map: &Vectrix, next_map: &mut Vectrix, neighbors: u32, i: &usize, j: &usize) {
    match &map[*i][*j] {
        Cell::Alive(_) => {
            // 1. Any live cell with fewer than 2 live neighbors dies, as if by underpopulation.
            if neighbors < 2 {
                next_map[*i][*j] = Cell::dead();
            }
            // 2. Any live cell with 2 or 3 live neighbors lives on to the next generation.
            if neighbors == 2 || neighbors == 3 {
                next_map[*i][*j] = Cell::alive();
            }
            // 3. Any live cell with more than 3 live neighbors dies, as if by overpopulation.
            if neighbors > 3 {
                next_map[*i][*j] = Cell::dead();
            }
        },
        Cell::Dead(_) => {
            // 4. Any dead cell with exactly 3 live neighbors becomes a live cell, as if by reproduction.
            if neighbors == 3 {
                next_map[*i][*j] = Cell::alive();
            }
        },
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
        Cell::Alive(_) => neighbors + 1,
        Cell::Dead(_) => neighbors,
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
        Cell::Alive(_) => neighbors + 1,
        Cell::Dead(_) => neighbors,
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
        Cell::Alive(_) => neighbors + 1,
        Cell::Dead(_) => neighbors,
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
        Cell::Alive(_) => neighbors + 1,
        Cell::Dead(_) => neighbors,
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
        Cell::Alive(_) => neighbors + 1,
        Cell::Dead(_) => neighbors,
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
        Cell::Alive(_) => neighbors + 1,
        Cell::Dead(_) => neighbors,
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
        Cell::Alive(_) => neighbors + 1,
        Cell::Dead(_) => neighbors,
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
        Cell::Alive(_) => neighbors + 1,
        Cell::Dead(_) => neighbors,
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
        Err(_) => return (vec![vec![Cell::dead(); 2]; 2], format!("[-] Failed to load map.")),
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
            map[i].push(Cell::alive());
            unit.clear();
        };

        // If unit is a dead cell, push False
        if unit.eq("[ ]") {
            map[i].push(Cell::dead());
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
                Cell::Alive(_) => "[x]",
                Cell::Dead(_) => "[ ]",
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

fn esc_key_pressed() -> bool {
    use crossterm::event::{self, Event, KeyCode};

    // poll(0) means it returns immediately with event availability information
    // using while instead of if to clear the pending events are processed (if many keys are pressed between ticks).
    while event::poll(std::time::Duration::from_millis(0)).unwrap() {
        match event::read().unwrap() {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Esc {
                    return true;
                }
            },
            _ => (),
        };
    }
    false
}

fn set_infinite_game(prev_state: &bool) -> (bool, String) {
    
    let new_state: bool = !prev_state;

    let message: String = match new_state {
        true => format!("Infinite game Enabled"),
        false => format!("Infinite game Disabled"),
    };
    
    return (new_state, message)
}

#[derive(Clone)]
struct Coordinates {
    row: usize,
    col: usize,
}

impl Coordinates {
    fn new(row: usize, col: usize) -> Coordinates {
        Coordinates {
            row,
            col,
        }
    }
    // more variants 
    // .
    // .
       
}

enum Spaceship {
    LightweightSpaceship(Vec<Coordinates>),
    Glider(Vec<Coordinates>),
}

impl Spaceship {
    fn glider(origin: Coordinates) -> Spaceship {
        let mut points = Vec::<Coordinates>::new();    

        let cell_1 = Coordinates::new(origin.row, origin.col);
        let cell_2 = Coordinates::new(origin.row + 1, origin.col);
        let cell_3 = Coordinates::new(origin.row + 1, origin.col + 2);
        let cell_4 = Coordinates::new(origin.row + 2, origin.col);
        let cell_5 = Coordinates::new(origin.row + 2, origin.col + 1);
        
        points.push(cell_1);
        points.push(cell_2);
        points.push(cell_3);
        points.push(cell_4);
        points.push(cell_5);
        
        Spaceship::Glider(points)
    }
    fn lightweight_spaceship(origin: Coordinates) -> Spaceship {
        let mut points = Vec::<Coordinates>::new();    

        let cell_1 = Coordinates::new(origin.row, origin.col + 1);
        let cell_2 = Coordinates::new(origin.row, origin.col + 2);
        let cell_3 = Coordinates::new(origin.row, origin.col + 4);
        let cell_4 = Coordinates::new(origin.row + 1, origin.col);
        let cell_5 = Coordinates::new(origin.row + 1, origin.col + 4);
        let cell_6 = Coordinates::new(origin.row + 2, origin.col + 4);
        let cell_7 = Coordinates::new(origin.row + 3, origin.col);
        let cell_8 = Coordinates::new(origin.row + 3, origin.col + 3);
        
        points.push(cell_1);
        points.push(cell_2);
        points.push(cell_3);
        points.push(cell_4);
        points.push(cell_5);
        points.push(cell_6);
        points.push(cell_7);
        points.push(cell_8);
        
        Spaceship::LightweightSpaceship(points)
    }

    fn get_points(pattern: Spaceship) -> Vec<Coordinates> {
        match pattern {
            Spaceship::Glider(points) => {
                points
            },
            Spaceship::LightweightSpaceship(points) => {
                points
            },
        }
    }
}

enum StillLife {
    FifteenBentPaperclip(Vec<Coordinates>),
    // more variants 
    // .
    // .
}

impl StillLife {
    fn fifteen_bent_paperclip(origin: Coordinates) -> StillLife {
        let mut coordinates = Vec::<Coordinates>::new();

        let cell_1 = Coordinates::new(origin.row, origin.col + 2);
        let cell_2 = Coordinates::new(origin.row, origin.col + 3);
        
        let cell_3 = Coordinates::new(origin.row + 1, origin.col + 1);
        let cell_4 = Coordinates::new(origin.row + 1, origin.col + 3);
        let cell_5 = Coordinates::new(origin.row + 1, origin.col + 4);
        let cell_6 = Coordinates::new(origin.row + 1, origin.col + 5);
        
        let cell_7 = Coordinates::new(origin.row + 2, origin.col);
        let cell_8 = Coordinates::new(origin.row + 2, origin.col + 6);
        
        let cell_9 = Coordinates::new(origin.row + 3, origin.col + 1);
        let cell_10 = Coordinates::new(origin.row + 3, origin.col + 2);
        let cell_11 = Coordinates::new(origin.row + 3, origin.col + 3);
        let cell_12 = Coordinates::new(origin.row + 3, origin.col + 4);
        let cell_13 = Coordinates::new(origin.row + 3, origin.col + 6);

        let cell_14 = Coordinates::new(origin.row + 4, origin.col + 3);
        let cell_15 = Coordinates::new(origin.row + 4, origin.col + 5);

        coordinates.push(cell_1);
        coordinates.push(cell_2);

        coordinates.push(cell_3);
        coordinates.push(cell_4);
        coordinates.push(cell_5);
        coordinates.push(cell_6);
        
        coordinates.push(cell_7);
        coordinates.push(cell_8);

        coordinates.push(cell_9);
        coordinates.push(cell_10);
        coordinates.push(cell_11);
        coordinates.push(cell_12);
        coordinates.push(cell_13);

        coordinates.push(cell_14);
        coordinates.push(cell_15);

        StillLife::FifteenBentPaperclip(coordinates)
    }

    fn get_points(pattern: StillLife) -> Vec<Coordinates> {
        match pattern {
            StillLife::FifteenBentPaperclip(points) => {
                points
            },
            // extra variants,
        }
    }
}



struct Map;

impl Map {
    // fn new() -> Vectrix {

    // }

    fn new() -> (Vectrix, String) {
        print_message(&String::from("Generate your map."), true);
        
        // No use for ABORTED
        let (rows, _aborted) = get_usize(&String::from("Rows:"), false);
        let (cols, _aborted) = get_usize(&String::from("Cols:"), false);

        if rows == 0 || cols == 0 {
            return (
                vec![vec![Cell::dead(); 10]; 10], 
                String::from(format!("[-] Invalid dimensions. 10 x 10 map created."))
            );
        }

        (
            vec![vec![Cell::dead(); cols]; rows], 
            String::from(format!("[+] {rows} x {cols} map created."))
        )
    }

    fn set_clear_cell(map: &mut Vectrix) -> String {
        let mut message = String::from("Set/Clear Cells");
        let message_loc = String::from("Enter Row and Column");
        // Default is "not edited"
        let mut edited: bool = false;

        loop {
            clear_console();
            print_header(vec!["Game of Life"]);
            print_map(&map, true, true);
            print_message(&message, true);
            print_message(&message_loc, true);
            
            let (row, aborted)  = get_usize(&String::from("Row:"), true);
            if aborted {
                break;
            }
            
            let (col, aborted) = get_usize(&String::from("Col:"), true);
            if aborted {
                break;
            }

            let row_len = map.len();
            let col_len = map[0].len();
            
            let filtered_row = row % row_len;
            let filtered_col = col % col_len;
            
            map[filtered_row][filtered_col] = map[filtered_row][filtered_col].not();
            
            match &map[filtered_row][filtered_col] {
                Cell::Alive(ch) => { 
                    message = format!("[{ch} ] Alive cell at [{filtered_row:>2}][{filtered_row:>2}]");
                },
                Cell::Dead(ch) => { 
                    message = format!("[{ch} ] Dead cell at [{filtered_row:>2}][{filtered_col:>2}]");
                },
            };

            // If we get here, then the map was edited
            edited =  true;
        }

        if edited {
            message = String::from("Map edited successfully.");
        } else {
            message = String::from("Aborted");
        }

        return message;
    }

    fn generate_spaceship(map: &mut Vectrix, pattern: Spaceship) -> String {
        let message: String;

        let points = match pattern {
            Spaceship::Glider(points) => {
                message = String::from("[+] Glider created.");
                points
            },
            Spaceship::LightweightSpaceship(points) => {
                message = String::from("[+] Lightweight Spaceship created.");
                points
            },
        };

        for point in points {
            let row_len = map.len();
            let col_len = map[0].len();

            let filtered_row = point.row % row_len;
            let filtered_col = point.col % col_len;

            map[filtered_row][filtered_col] = Cell::alive();
        }

        message
    }

    fn generate_still_life(map: &mut Vectrix, pattern: StillLife) -> String {
        let message: String;

        let points = match pattern {
            StillLife::FifteenBentPaperclip(points) => {
                message = String::from("[+] Fifteen Bent Paperclip created.");
                points
            },
        };

        for point in points {
            let row_len = map.len();
            let col_len = map[0].len();

            let filtered_row = point.row % row_len;
            let filtered_col = point.col % col_len;

            map[filtered_row][filtered_col] = Cell::alive();
        }

        message
    }

    fn generate_pattern(map: &mut Vectrix, points: Vec<Coordinates>) -> String {
        for point in points {
            let row_len = map.len();
            let col_len = map[0].len();

            let filtered_row = point.row % row_len;
            let filtered_col = point.col % col_len;

            map[filtered_row][filtered_col] = Cell::alive();
        }

        String::from("[+] Pattern generated.")
    }
}

// enum StillLifes {
//     //
// }

// enum Oscillators {
//     //
// }

// enum Methuselahs {
//     //
// }

// enum Guns {
//     //
// }

// fn print_rules() {

// }