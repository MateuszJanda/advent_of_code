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

fn move_left(
    buffer: &Vec<Vec<char>>,
    block: &mut Vec<Vec<char>>,
    start_block: Option<usize>,
    start_buff: Option<usize>,
) {
    for line in block.iter() {
        if line[FIRST_COL] != ' ' {
            return;
        }
    }

    if is_left_obstacle(buffer, block, start_block, start_buff) {
        return;
    }

    for line in block {
        line.rotate_left(1);
    }
}

fn move_right(
    buffer: &Vec<Vec<char>>,
    block: &mut Vec<Vec<char>>,
    start_block: Option<usize>,
    start_buff: Option<usize>,
) {
    for line in block.iter() {
        if line[LAST_COL] != ' ' {
            return;
        }
    }

    if is_right_obstacle(buffer, block, start_block, start_buff) {
        return;
    }

    for line in block {
        line.rotate_right(1);
    }
}

fn is_bottom_obstacle(
    buffer: &Vec<Vec<char>>,
    block: &Vec<Vec<char>>,
    start_block: Option<usize>,
    start_buff: Option<usize>,
) -> bool {
    if buffer.is_empty() {
        return true;
    }

    let (start_block, start_buff) = match (start_block, start_buff) {
        (None, None) => (block.len() - 1, buffer.len()),
        (Some(_), Some(0)) => return true,
        (Some(s_block), Some(s_buff)) => (s_block, s_buff),
        _ => panic!("Not supported case"),
    };

    for y_shift in 0..(block.len() - start_block) {
        for x in 0..block[0].len() {
            if block[start_block + y_shift][x] == '#' && buffer[start_buff - y_shift - 1][x] == '#'
            {
                return true;
            }
        }
    }

    false
}

fn is_left_obstacle(
    buffer: &Vec<Vec<char>>,
    block: &Vec<Vec<char>>,
    start_block: Option<usize>,
    start_buff: Option<usize>,
) -> bool {
    let (start_block, start_buff) = match (start_block, start_buff) {
        (None, None) => return false,
        (Some(s_block), Some(s_buff)) => (s_block, s_buff),
        _ => panic!("Not supported case"),
    };

    for y_shift in 0..(block.len() - start_block) {
        for x in 1..block[0].len() {
            if block[start_block + y_shift][x] == '#' && buffer[start_buff - y_shift][x - 1] == '#'
            {
                return true;
            }
        }
    }

    false
}

fn is_right_obstacle(
    buffer: &Vec<Vec<char>>,
    block: &Vec<Vec<char>>,
    start_block: Option<usize>,
    start_buff: Option<usize>,
) -> bool {
    let (start_block, start_buff) = match (start_block, start_buff) {
        (None, None) => return false,
        (Some(s_block), Some(s_buff)) => (s_block, s_buff),
        _ => panic!("Not supported case"),
    };

    for y_shift in 0..(block.len() - start_block) {
        for x in 0..block[0].len() - 1 {
            if block[start_block + y_shift][x] == '#' && buffer[start_buff - y_shift][x + 1] == '#'
            {
                return true;
            }
        }
    }

    false
}

fn merge(
    buffer: &mut Vec<Vec<char>>,
    block: &Vec<Vec<char>>,
    start_block: Option<usize>,
    start_buff: Option<usize>,
) {
    if buffer.is_empty() || start_buff.is_none() {
        for line in block.iter().rev() {
            buffer.push(line.clone());
        }
        return;
    }

    let start_buff = start_buff.unwrap();
    let start_block = start_block.unwrap();

    for y_shift in 0..(block.len() - start_block) {
        for x in 0..block[0].len() {
            if block[start_block + y_shift][x] == '#' {
                buffer[start_buff - y_shift][x] = '#';
            }
        }
    }

    for y in (0..start_block).rev() {
        buffer.push(block[y].clone());
    }
}

#[allow(dead_code, unused)]
fn print_buffer(buffer: &Vec<Vec<char>>) {
    println!("-----------");
    for line in buffer.iter().rev() {
        println!("|{}|", line.iter().collect::<String>().replace(" ", "."));
    }
}

#[allow(dead_code, unused)]
fn print_block(block: &Vec<Vec<char>>) {
    println!("-----------");
    for line in block.iter() {
        println!(
            "|{}|",
            line.iter()
                .collect::<String>()
                .replace(" ", ".")
                .replace("#", "@")
        );
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
            vec![' ',' ','#',' ',' ',' ',' '],
            vec![' ',' ','#',' ',' ',' ',' '],
            vec![' ',' ','#',' ',' ',' ',' '],
            vec![' ',' ','#',' ',' ',' ',' '],
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
    for (i, dir) in commands.chars().cycle().enumerate() {
        // println!("lift {} {} {}", lift, i, dir);
        if block_counter == NUM_OF_ROCKS {
            break;
        }

        let start_block = match lift <= LIFT {
            true => None,
            false => Some(std::cmp::max(0, block.len() as i32 - 1 - (lift - (LIFT + 1))) as usize),
        };

        let start_buff = match lift <= LIFT {
            true => None,
            false => {
                let l = lift - (LIFT + 1);

                if l < block.len() as i32 {
                    Some(buffer.len() - 1)
                } else {
                    let idx = buffer.len() as i32 - 1 - (l - block.len() as i32);
                    Some(idx as usize)
                }
            }
        };

        match dir {
            '<' => move_left(&buffer, &mut block, start_block, start_buff),
            '>' => move_right(&buffer, &mut block, start_block, start_buff),
            _ => panic!("Unsupported dir"),
        }
        // print_block(&block);

        if lift < LIFT {
            lift += 1;
            continue;
        }

        if is_bottom_obstacle(&buffer, &block, start_block, start_buff) {
            merge(&mut buffer, &block, start_block, start_buff);

            // print_buffer(&buffer);

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
