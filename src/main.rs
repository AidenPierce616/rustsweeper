use std::collections::HashMap;
use rand::seq::SliceRandom;

use ansi_term::{ANSIGenericString, Colour::{Black, White, RGB}};

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn mine_board(board: &mut Vec<Vec<Cell>>, boardsize: usize) {
    let mut clear_indexes: Vec<(usize, usize)> = vec![];
    for (row_number, row) in board.iter_mut().enumerate() {
        let mut indexes: Vec<usize> = vec![];
        for i in 0..boardsize {
            indexes.push(i);
        }
        let indexes = indexes.clone();

        let mine_number = boardsize / 8;

        let choices: Vec<&usize> = indexes
            .choose_multiple(&mut rand::thread_rng(), mine_number)
            .collect();

        for i in choices.iter() {
            row[**i].element = 'M';
        }
        for i in indexes.iter() {
            match choices.iter().position(|&r| r == i) {
                Some(_) => {},
                None => {
                    clear_indexes.push((row_number, *i));
                }
            }
        }
    }
}

fn display_board(board: &Vec<Vec<Cell>>, board_objects_map:&HashMap<char, ANSIGenericString<'static, str>>, boardsize: usize) {
    print!("    ");
    for i in 1..boardsize+1 {
        let temp: String;
        if i < 10 {
            temp = format!("0{} ", i);
        } else {
            temp = format!("{} ", i);
        }
        print!("{}", temp);
    }
    println!();
    for (i, row) in board.iter().enumerate() {
        if i > 8 {
            print!("{:3} ", i+1);
        } else {
            let temp_str = String::from("0") + &String::from(char::from_digit((i+1).try_into().unwrap(), 10).expect("Fuck"));
            print!(" {} ", temp_str);
        }
        for (_, cell) in row.iter().enumerate() {
            if cell.flagged == false {
                if cell.hidden == true {
                    print!("{}", board_objects_map.get(&'#').expect("Fuck"));
                } else {
                    if cell.element == '0'{
                        print!("{}", board_objects_map.get(&' ').expect("Fuck"))
                    } else {
                        let temp_str = board_objects_map.get(&cell.element).expect("Fuck");
                        print!("{}", temp_str);
                    }   
                }
            } else {
                print!("{}", board_objects_map.get(&'⚑').expect("Fuck"));
            }
        }
        if i > 8 {
            print!("{:3} ", i+1);
        } else {
            let temp_str = String::from("0") + &String::from(char::from_digit((i+1).try_into().unwrap(), 10).expect("Fuck"));
            print!(" {} ", temp_str);
        }
        println!("");
    }
    print!("    ");
    for i in 1..boardsize+1 {
        let temp: String;
        if i < 10 {
            temp = format!("0{} ", i);
        } else {
            temp = format!("{} ", i);
        }
        print!("{}", temp);
    }
    println!("");
}

fn get_int_in_range_from_user(l: i32,u: i32, msg: String) -> i32 {
    println!("{}", msg);
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let number = match trimmed.parse::<i32>() {
        Ok(i) => i,
        Err(..) => -1,
    };

    if number == -1 || number < l || number > u {
        return get_int_in_range_from_user(l, u, msg);
    }
    number
}

fn get_coord_from_user(boardsize: usize) -> (i32, i32) {
    println!("Enter coordinates");
    let row = get_int_in_range_from_user(1, (boardsize+1).try_into().unwrap(), String::from("Enter row coordinate: "));
    let col = get_int_in_range_from_user(1, (boardsize+1).try_into().unwrap(), String::from("Enter column coordinate: "));
    (row-1, col-1)
}

fn get_option_from_user(option1: char, option2: char) -> char{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let byte: char = input.bytes().nth(0).expect("no byte read") as char;
    if byte != option1 && byte != option2 {
        return get_option_from_user(option1, option2);
    }
    byte
}

fn get_around_cell(coords: [usize; 2], board: &Vec<Vec<Cell>>, boardsize: usize) -> Vec<(char, usize, usize)>  {
    let mut cells: Vec<(char, usize, usize)> =  vec![];
    let iterator = [coords[0] as i32, coords[1] as i32];
    for i in iterator[0]-1..=iterator[0]+1 {
        for j in iterator[1]-1..=iterator[1]+1 {
            if !(i == 0 && j == 0) && i >= 0 && j >= 0 && i < boardsize as i32 && j < boardsize as i32{
                cells.push((board[i as usize][j as usize].element, i as usize, j as usize));
            }
        }   
    }
    cells
}

fn make_numbers(board: &mut Vec<Vec<Cell>>, boardsize: usize) {
    let mut board_copy = board.clone();
    for (row_number, row) in board.iter().enumerate() {
        for (column_number, cell) in row.iter().enumerate() {
            let around = get_around_cell([row_number,column_number], &board, boardsize);
            let mut number = 0;
            for i in around.iter() {
                if i.0 == 'M' {
                    number += 1;
                }
            }
            if cell.element != 'M'{
                board_copy[row_number][column_number].element = char::from_digit(number, 10).expect("Fuck");
            }
        }
    }
    *board = board_copy.clone();
}

fn deobfuscate_board(board: &mut Vec<Vec<Cell>>, row_number:usize, column_number:usize, boardsize: usize) {
    let mut to_check = vec![];
    if board[row_number][column_number].element == '0' {
        to_check.push((row_number, column_number));
    }
    let mut next_to_check: Vec<(usize,usize)> = Vec::new();
    let mut prev_checked: Vec<(usize,usize)> = Vec::new();
    while !to_check.is_empty() {
        for i in to_check.iter() {
            let around = get_around_cell([i.0,i.1], board, boardsize);
            for j in around.iter() {
                let curr_cell = (j.1,j.2);
                if !prev_checked.contains(&curr_cell) {
                    prev_checked.push(curr_cell);
                    if j.0 == '0' {
                        next_to_check.push(curr_cell);
                        board[j.1][j.2].hidden = false;
                    } else if j.0 != '0' && j.0 != 'M' {
                        board[j.1][j.2].hidden = false;
                    }
                }
            }
        }
        to_check = next_to_check.clone();
        next_to_check = vec![];
    }
}

fn event(row_number:i32, column_number:i32, board: &mut Vec<Vec<Cell>>, boardsize: usize) -> char {
    let temp_cell = board[row_number as usize][column_number as usize].element; // sometimes incorrectly detected for some reason
    if temp_cell == 'M' {
        'D'
    } else if temp_cell != '0'{
        board[row_number as usize][column_number as usize].hidden = false;
        'F'
    } else {
        board[row_number as usize][column_number as usize].hidden = false;
        deobfuscate_board(board, row_number as usize, column_number as usize, boardsize);
        'F'
    }
}

fn flag(board: &mut Vec<Vec<Cell>>, row_number:usize, column_number:usize) {
    if !board[row_number][column_number].flagged {
        board[row_number][column_number].flagged = true;
    } else {
        board[row_number][column_number].flagged = false;
    }
}

fn won(board: &Vec<Vec<Cell>>) -> bool {
    for i in board.iter() {
        for &j in i.iter() {
            if j.hidden == true && j.element != 'M' {
                return false;
            }
        }
    }
    true
}

#[derive(Copy, Clone)]
struct Cell {
    hidden: bool,
    element: char,
    flagged: bool,
}

fn main() {
    clear();
    let boardsize = get_int_in_range_from_user(0, 1000, String::from("Enter board size"));
    let mut board = vec![vec![Cell {
        hidden : true,
        element: '0',
        flagged: false
    }; boardsize as usize]; boardsize as usize];

    let board_objects_map: HashMap<char, ANSIGenericString<'static, str>> = HashMap::from([
        ('M', RGB(0, 0, 0).on(White).bold().paint(" 🟐 ")),
        ('1', RGB(6, 3, 255).on(White).bold().paint(" 1 ")),
        ('2', RGB(3, 122, 6).on(White).bold().paint(" 2 ")),
        ('3', RGB(254, 0, 0).on(White).bold().paint(" 3 ")),
        ('4', RGB(0, 0, 132).on(White).bold().paint(" 4 ")),
        ('5', RGB(130, 1, 2).on(White).bold().paint(" 5 ")),
        ('6', RGB(2, 127, 130).on(White).bold().paint(" 6 ")),
        ('7', RGB(0, 0, 0).on(White).bold().paint(" 7 ")),
        ('8', RGB(125, 125, 125).on(White).bold().paint(" 8 ")),
        ('#', Black.on(Black).bold().paint("   ")),
        ('⚑', White.on(Black).bold().paint(" ⚑ ")),
        (' ', White.on(White).bold().paint("   "))
    ]);

    mine_board(&mut board, boardsize.try_into().unwrap());
    make_numbers(&mut board, boardsize.try_into().unwrap());
    loop {
        display_board(&board, &board_objects_map, boardsize.try_into().unwrap());
        let (row_number, column_number) = get_coord_from_user(boardsize.try_into().unwrap());
        println!("Pick what to do: flag or press (f/p)");
        let choice = get_option_from_user('f', 'p');
        clear();
        if choice == 'p' {
            let event = event(row_number, column_number, &mut board, boardsize.try_into().unwrap());
            if event == 'D' {
                clear();
                for i in board.iter_mut() {
                    for j in i.iter_mut() {
                        j.hidden = false;
                    }
                }
                display_board(&board, &board_objects_map, boardsize.try_into().unwrap());
                println!("You died.");
                return;
            } else {
                clear();
            }
            if won(&board) {
                for i in board.iter_mut() {
                    for j in i.iter_mut() {
                        j.hidden = false;
                    }
                }
                display_board(&board, &board_objects_map, boardsize.try_into().unwrap());
                println!("You win!");
                return;
            }
        } else {
            flag(&mut board, row_number as usize, column_number as usize);
            clear();
        }
    }
}
