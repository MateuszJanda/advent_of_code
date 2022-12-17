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

fn main() {
    #[rustfmt::skip]
    let blocks = vec![
        vec![
            vec!['#','#','#','#']
        ],

        vec![
            vec!['.','#','.'],
            vec!['#','#','#'],
            vec!['.','#','.']
        ],

        vec![
            vec!['.','.','#'],
            vec!['.','.','#'],
            vec!['#','#','#']
        ],

        vec![
            vec!['#'],
            vec!['#'],
            vec!['#'],
            vec!['#'],
        ],

        vec![
            vec!['#','#'],
            vec!['#','#'],
        ]

    ];

    let commands = read_string().unwrap();
}
