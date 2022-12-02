// Author:  mateusz.janda@gmail.com
// Ad maiorem Dei gloriam

use std::io;

pub struct Solution {}

impl Solution {}

fn read_num() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let num: i32 = line.strip_suffix("\n").unwrap().parse().unwrap();

    return num;
}

fn read_nums() -> Vec<i32> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let nums: Vec<i32> = line
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();

    return nums;
}

fn print_nums(vec: Vec<i32>) {
    for value in vec {
        print!("{},", value);
    }
    println!();
}

fn read_string() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.strip_suffix("\n").unwrap().to_string();
}

fn read_strings() -> Vec<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let words: Vec<String> = line
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();

    return words;
}

fn read_linked_list() -> Option<Box<ListNode>> {
    let nums = read_nums();

    if nums.len() == 0 {
        return None;
    }

    let mut head = Some(Box::new(ListNode::new(nums[0])));
    let mut parent = &mut head;

    for i in 1..nums.len() {
        if let Some(ref mut n) = parent {
            n.next = Some(Box::new(ListNode::new(nums[i])));
            parent = &mut n.next;
        }
    }

    return head;
}

fn print_linked_list(head: Option<Box<ListNode>>) {
    let mut next = head;

    while next.is_some() {
        print!("{},", next.clone().unwrap().val);
        next = next.unwrap().next;
    }
    println!("");
}

// Binary tree using BFS for level order traversal
fn read_tree_in_level_order() -> Option<Rc<RefCell<TreeNode>>> {
    let vals = read_strings();

    if vals.is_empty() || vals[0] == "null" || vals[0].is_empty() {
        return None;
    }

    let mut index = 0;
    let mut queue = VecDeque::new();
    let root = Some(Rc::new(RefCell::new(TreeNode::new(
        vals[index].parse().unwrap(),
    ))));
    queue.push_back(root.clone());

    index += 1;
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if let Some(n) = node {
            if index >= vals.len() {
                break;
            }

            let mut tree_node = n.borrow_mut();
            tree_node.left = match vals[index] != "null" {
                true => {
                    let left = Some(Rc::new(RefCell::new(TreeNode::new(
                        vals[index].parse().unwrap(),
                    ))));
                    queue.push_back(left.clone());
                    left
                }
                false => None,
            };
            index += 1;

            if index >= vals.len() {
                break;
            }

            tree_node.right = match vals[index] != "null" {
                true => {
                    let right = Some(Rc::new(RefCell::new(TreeNode::new(
                        vals[index].parse().unwrap(),
                    ))));
                    queue.push_back(right.clone());
                    right
                }
                false => None,
            };
            index += 1;
        };
    }

    return root;
}

fn print_tree_in_level_order(root: Option<Rc<RefCell<TreeNode>>>) {
    let mut queue = VecDeque::new();
    let mut result = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        match node {
            Some(n) => {
                let tree_node = n.borrow();
                result.push_back(tree_node.val.to_string());

                let left = tree_node.left.clone();
                queue.push_back(left);

                let right = tree_node.right.clone();
                queue.push_back(right);
            }
            None => result.push_back("null".to_string()),
        };
    }

    while let Some(last) = result.back() {
        if last == &"null" {
            result.pop_back();
        } else {
            break;
        }
    }

    for i in 0..result.len() {
        print!("{},", result[i]);
    }

    println!("");
}

fn main() {
    println!("Hello world!")
}
