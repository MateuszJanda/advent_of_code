// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

fn read_string() -> Option<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.strip_suffix("\n") {
        None => None,
        Some(stripped_line) => {
            if stripped_line.is_empty() {
                return None;
            }

            Some(stripped_line.to_string())
        }
    }
}

const FIRST_COL: usize = 0;
const LAST_COL: usize = 6;

fn move_left(block: &mut Vec<Vec<char>>) {
    for line in block.iter() {
        if line[FIRST_COL] != ' ' {
            return;
        }
    }

    for line in block {
        line.rotate_left(1);
    }
}

fn move_right(block: &mut Vec<Vec<char>>) {
    for line in block.iter() {
        if line[LAST_COL] != ' ' {
            return;
        }
    }

    for line in block {
        line.rotate_right(1);
    }
}

fn is_floor(
    buffer: &Vec<Vec<char>>,
    block: &Vec<Vec<char>>,
    start_block_line: usize,
    start_buff_line: usize,
) -> bool {
    if buffer.is_empty() {
        return true;
    }

    for y_shift in 0..(block.len() - start_block_line) {
        for x in 0..block[0].len() {
            if buffer[start_buff_line + y_shift][x] == '#'
                && block[start_block_line + y_shift][x] == '#'
            {
                return true;
            }
        }
    }

    false
}

// fn is_wall(
//     buffer: &Vec<Vec<char>>,
//     block: &Vec<Vec<char>>,
//     start_block_line: usize,
//     start_buff_line: usize,
// ) -> bool {
//     false
// }

fn merge(
    buffer: &mut Vec<Vec<char>>,
    block: &Vec<Vec<char>>,
    start_block_line: usize,
    start_buff_line: usize,
) {
    println!("start {} {} ", start_block_line, start_buff_line);

    if buffer.is_empty() {
        for line in block.iter().rev() {
            buffer.push(line.clone());
        }

        return;
    }

    for y in (0..start_block_line).rev() {
        buffer.push(block[y].clone());
    }

    for y_shift in 0..(block.len() - start_block_line) {
        for x in 0..block[0].len() {
            if block[start_block_line + y_shift][x] == '#' {
                buffer[start_buff_line + y_shift][x] = '#';
            }
        }
    }
}

#[allow(dead_code, unused)]
fn print_buffer(buffer: &Vec<Vec<char>>) {
    for line in buffer.iter().rev() {
        println!("|{}|", line.iter().collect::<String>().replace(" ", "."));
    }
}

const NUM_OF_ROCKS: usize = 2022;
const LIFT: i32 = 3;

fn main() {
    #[rustfmt::skip]
    let blocks = vec![
        vec![
            vec![' ',' ','#','#','#','#',' ']
        ],

        vec![
            vec![' ',' ',' ','#',' ',' ',' '],
            vec![' ',' ','#','#','#',' ',' '],
            vec![' ',' ',' ','#',' ',' ',' '],
        ],

        vec![
            vec![' ',' ',' ',' ','#',' ',' '],
            vec![' ',' ',' ',' ','#',' ',' '],
            vec![' ',' ','#','#','#',' ',' '],
            ],

        vec![
            vec![' ',' ',' ','#',' ',' ',' '],
            vec![' ',' ',' ','#',' ',' ',' '],
            vec![' ',' ',' ','#',' ',' ',' '],
            vec![' ',' ',' ','#',' ',' ',' '],
        ],

        vec![
            vec![' ',' ','#','#',' ',' ',' '],
            vec![' ',' ','#','#',' ',' ',' '],
        ]

    ];

    let mut buffer = vec![];
    let mut block_num = 0;
    let mut block = blocks[block_num].clone();
    let mut block_counter = 0;
    let mut lift = 0;

    let commands = read_string().unwrap();
    for dir in commands.chars() {
        if block_counter >= NUM_OF_ROCKS {
            break;
        }

        let start_block_line = match lift >= LIFT {
            true => std::cmp::max(0, block.len() as i32 - 1 - (lift - LIFT)) as usize,
            false => block.len() - 1,
        };

        let start_buff_line = match lift >= LIFT {
            true => std::cmp::max(0, buffer.len() as i32 - 1 - (lift - LIFT)) as usize,
            false => 0,
        };

        match dir {
            '<' => move_left(&mut block),
            '>' => move_right(&mut block),
            _ => panic!("Unsupported dir"),
        }

        println!("lift {}", lift);
        if lift < LIFT {
            lift += 1;
            continue;
        }

        if is_floor(&buffer, &block, start_block_line, start_buff_line) {
            merge(&mut buffer, &block, start_block_line, start_buff_line);

            print_buffer(&buffer);

            block_counter += 1;
            block_num = (block_num + 1) % blocks.len();
            block = blocks[block_num].clone();
            lift = 0;
        } else {
            lift += 1;
        }
    }

    println!("{}", buffer.len());
}
