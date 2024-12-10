mod heap;

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::rc::Rc;
use text_io::read;
use crate::heap::*;

#[derive(Debug, PartialEq)]
struct DiskBlock {
    space: usize,
    pos: usize,
}

#[derive(Debug, PartialEq)]
struct Node {
    space: usize,
    val: Option<usize>,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug, Default)]
struct DiskSpace {
    empty: Option<Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl DiskSpace {

    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut current = self.head.clone();

        while let Some(node) = current {
            let node = node.borrow();
            if let Some(val) = node.val {
                result.push_str(&format!("{}:{};", val, node.space));
            } else {
                result.push_str(&format!("E:{};", node.space));
            }

            if let Some(next_node_rc) = &node.next {
                current = Some(next_node_rc.clone());
            } else {
                break;
            }
        }
        result
    }

    fn push_back(&mut self, space: usize, val: Option<usize>) -> Option<Rc<RefCell<Node>>> {
        let new_node = Rc::new(RefCell::new(
            Node{space, val, prev: self.tail.clone(), next: None }
        ));

        if let Some(tail) = &self.tail.take() {
            tail.borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node.clone());

        if self.empty.is_none() && val == None {
            self.empty = Some(new_node.clone());
        }

        Some(new_node.clone())
    }

    fn pop_back(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(tail) = self.tail.take() {
            let prev_node = tail.borrow_mut().prev.take();
            if let Some(prev_node) = prev_node {
                prev_node.borrow_mut().next = None;
                self.tail = Some(prev_node);
            } else {
                self.head = None;
                self.empty = None;
            }
            return Some(tail.to_owned())
        }
        None
    }

    fn insert_after(&mut self, node: Option<Rc<RefCell<Node>>>, space: usize, val: Option<usize>) -> Option<Rc<RefCell<Node>>> {
        let node = node?; // Return early if None

        let next = node.borrow().next.clone();

        let new_node = Rc::new(RefCell::new(
        Node{space, val, prev: Some(node.clone()), next: next.clone() }
        ));

        if let Some(next_node) = next {
            next_node.borrow_mut().prev = Some(new_node.clone());
        } else {
            self.tail = Some(new_node.clone());
        }

        node.borrow_mut().next = Some(new_node.clone());

        Some(new_node)
    }

    fn split(&mut self, node: Option<Rc<RefCell<Node>>>, space: usize, val: Option<usize>) -> Option<Rc<RefCell<Node>>> {
        let node = node?;

        let mut borrowed_node = node.borrow_mut();
        if space < borrowed_node.space {
            let rest = borrowed_node.space - space;
            borrowed_node.space = space;
            borrowed_node.val = val;
            drop(borrowed_node);

            return self.insert_after(Some(node.clone()), rest, None)
        }
        None
    }

    fn get_empty_after(&mut self, node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut current = node.clone();

        while let Some(current_node) = current {
            let current_node_ref = current_node.borrow();
            if let Some(next_node) = &current_node_ref.next {
                current = Some(next_node.clone());
                let current_node_ref = current.clone().unwrap();
                let current_node_ref = current_node_ref.borrow();
                if current_node_ref.val.is_none() {
                    return current.clone();
                }
            } else {
                return None
            }

        }
        None
    }

    /// Rearrange the file blocks one at a time from the end of the disk
    /// to the leftmost free space block.
    /// Returns the first empty block.
    fn rearrange(&mut self) {
        while !self.is_arranged() {
            if let Some(tail) = self.pop_back() {
                let mut borrowed_tail = tail.borrow_mut();

                if borrowed_tail.val.is_none() {
                    continue
                }

                while borrowed_tail.space != 0 {
                    if let Some(empty_node) = self.empty.clone() {
                        let mut current_empty = empty_node.borrow_mut();

                        if borrowed_tail.space >= current_empty.space {
                            current_empty.val = borrowed_tail.val;
                            borrowed_tail.space -= current_empty.space;
                            drop(current_empty);
                            self.empty = self.get_empty_after(Some(empty_node.clone()));
                        } else if let Some(val) = borrowed_tail.val {
                            drop(current_empty);
                            self.empty = self.split(Some(empty_node.clone()), borrowed_tail.space, Some(val));
                            borrowed_tail.space = 0;
                        } else {
                            panic!("Error: rearranging empty block!");
                        }
                    } else {
                        self.empty = self.push_back(9, None);
                    }
                }
            }
        }
    }

    fn is_arranged(&self) -> bool {
        let mut current = self.empty.clone();
        while let Some(node) = current {
            let node = node.borrow();
            if !node.val.is_none() {
                return false;
            }

            if let Some(next_node_rc) = &node.next {
                current = Some(next_node_rc.clone());
            } else {
                break;
            }
        }
        true
    }

    fn get_checksum(&self) -> usize {
        let mut result: usize = 0;
        let mut idx: usize = 0;
        let mut current = self.head.clone();

        while let Some(node) = current {
            let node = node.borrow();
            if let Some(val) = node.val {
                result += (idx..idx + node.space).map(|i| val * i).sum::<usize>();
            }
            idx += node.space;

            if let Some(next_node_rc) = &node.next {
                current = Some(next_node_rc.clone());
            } else {
                break;
            }
        }
        result
    }
}

/// [Triangular numbers](https://en.wikipedia.org/wiki/Triangular_number) offset by two.
/// Files can be a max size of 9 so we only need the first 10 values, including zero to make
/// indexing easier.
const EXTRA: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];
pub fn rearrange_with_whole_blocks(disk: &[usize]) -> usize {
    let mut block = 0; // the position of the block
    let mut checksum = 0;
    let mut free: Vec<_> = (0..10).map(|_| MinHeap::with_capacity(1_000)).collect();

    let mut is_empty = false;
    for (_, &size) in disk.iter().enumerate() {
        if is_empty && size > 0 {
            free[size].push(block, ());
        }
        is_empty = !is_empty;

        block += size;
    }

    is_empty = true;
    for (index, &size) in disk.iter().enumerate().rev() {
        is_empty = !is_empty;
        block -= size;

        // skip free blocks
        if is_empty {
            continue;
        }

        let mut next_block = block;
        let mut next_index = usize::MAX;

        for i in size..free.len() {
            if let Some((&first, ())) = free[i].peek() {
                if first < next_block {
                    next_block = first;
                    next_index = i;
                }
            }
        }

        if !free.is_empty() {
            let last = free.len() - 1;
            if let Some((&first, ())) = free[last].peek() {
                if first > block {
                    free.pop();
                }
            }
        }

        let id = index / 2;
        let extra = next_block * size + EXTRA[size];
        checksum += id * extra;

        if next_index != usize::MAX {
            free[next_index].pop();
            if size < next_index {
                free[next_index - size].push(next_block + size, ());
            }
        }
    }

    checksum
}

fn main() {
    let mut disk = DiskSpace::default();

    let input: String = read!("{}\n");
    let input = input.trim().to_owned();

    let blocks: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut is_empty = false;
    let mut block_val = 0;
    for &space in &blocks {
        disk.push_back(space, if is_empty { None } else { Some(block_val) });
        is_empty = !is_empty;
        block_val += if is_empty {0} else {1};
    }

    disk.rearrange();
    rearrange_with_whole_blocks(&blocks);
    println!("The checksum is: {}", disk.get_checksum());
    println!("The checksum for arrangement wiht the whole blocks it: {}", rearrange_with_whole_blocks(&blocks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_empty() {
        let disk = DiskSpace::default();
        assert_eq!(disk.to_string(), "");
        assert!(disk.head.is_none());
        assert!(disk.tail.is_none());
        assert!(disk.empty.is_none());
    }

    #[test]
    fn test_disk_push_basic() {
        let mut disk = DiskSpace::default();
        disk.push_back(4,Some(0));
        assert_eq!(disk.to_string(), "0:4;");
        assert!(disk.empty.is_none());
    }

    #[test]
    fn test_disk_push() {
        let mut disk = DiskSpace::default();
        disk.push_back(4,Some(0));
        disk.push_back(5,None);
        disk.push_back(6,Some(2));
        assert_eq!(disk.to_string(), "0:4;E:5;2:6;");
        assert!(!disk.empty.is_none());
    }

    #[test]
    fn test_disk_insert() {
        let mut disk = DiskSpace::default();
        let node = disk.push_back(4,Some(0));
        disk.push_back(5,Some(1));
        disk.push_back(6,Some(2));
        disk.insert_after(node, 3, Some(3));
        disk.push_back(5,Some(4));
        assert_eq!(disk.to_string(), "0:4;3:3;1:5;2:6;4:5;");
    }

    #[test]
    fn test_disk_pop_basic() {
        let mut disk = DiskSpace::default();
        disk.push_back(5,Some(1));
        disk.pop_back();
        assert_eq!(disk.to_string(), "");
        assert!(disk.head.is_none());
        assert!(disk.tail.is_none());
        assert!(disk.empty.is_none());
    }

    #[test]
    fn test_disk_pop() {
        let mut disk = DiskSpace::default();
        let node1 = disk.push_back(4,Some(0));
        disk.push_back(5,Some(1));
        disk.insert_after(node1, 3, Some(3));
        assert_eq!(disk.to_string(), "0:4;3:3;1:5;");
        disk.pop_back();
        assert_eq!(disk.to_string(), "0:4;3:3;");
        disk.pop_back();
        assert_eq!(disk.to_string(), "0:4;");
        disk.pop_back();
        assert_eq!(disk.to_string(), "");
    }

    #[test]
    fn test_disk_remove_basic() {
        let mut disk = DiskSpace::default();
        disk.push_back(5,Some(1));
        let pop_node = disk.push_back(5,Some(2));
        disk.push_back(5,Some(3));
        assert_eq!(disk.to_string(), "1:5;2:5;3:5;");
        disk.pop_node(pop_node);
        assert_eq!(disk.to_string(), "1:5;3:5;");
    }

    #[test]
    fn test_disk_split() {
        let mut disk = DiskSpace::default();
        let node1 = disk.push_back(4, None);
        disk.push_back(5, Some(1));
        assert_eq!(disk.to_string(), "E:4;1:5;");
        disk.split(node1, 2, Some(2));
        assert_eq!(disk.to_string(), "2:2;E:2;1:5;");
    }

    #[test]
    fn test_disk_rearrange_basic() {
        let mut disk = DiskSpace::default();
        disk.push_back(5,Some(1));
        disk.push_back(5,None);
        disk.push_back(5,Some(2));
        assert_eq!(disk.to_string(), "1:5;E:5;2:5;");
        disk.rearrange();
        assert_eq!(disk.to_string(), "1:5;2:5;");
    }

    #[test]
    fn test_disk_rearrange() {
        let mut disk = DiskSpace::default();
        disk.push_back(5,Some(1));
        disk.push_back(5,None);
        disk.push_back(5,Some(2));
        disk.push_back(5,None);
        disk.push_back(5,Some(3));
        disk.push_back(5,None);
        disk.push_back(13,Some(4));
        assert_eq!(disk.to_string(), "1:5;E:5;2:5;E:5;3:5;E:5;4:13;");
        disk.rearrange();
        assert_eq!(disk.to_string(), "1:5;4:5;2:5;4:5;3:5;4:3;E:2;");
    }

    #[test]
    fn test_disk_rearrange_complex() {
        let mut disk = DiskSpace::default();
        disk.push_back(5,Some(1));
        disk.push_back(5,None);
        disk.push_back(5,Some(2));
        disk.push_back(5,None);
        disk.push_back(5,Some(3));
        disk.push_back(5,None);
        disk.push_back(7,Some(4));
        assert_eq!(disk.to_string(), "1:5;E:5;2:5;E:5;3:5;E:5;4:7;");
        disk.rearrange();
        assert_eq!(disk.to_string(), "1:5;4:5;2:5;4:2;3:3;3:2;E:7;");
    }

    #[test]
    fn test_rearrange_small_input() {
        let mut disk = DiskSpace::default();
        let input: &str = "2333133121414131402";
        let input = input.trim().to_owned();

        let blocks: Vec<usize> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let mut is_empty = false;
        let mut block_val = 0;
        for space in blocks {
            disk.push_back(space, if is_empty { None } else { Some(block_val) });
            is_empty = !is_empty;
            block_val += if is_empty {0} else {1};
        }

        disk.rearrange();
        assert_eq!(disk.get_checksum(), 1928);
    }

    #[test]
    fn test_rearrange_with_whole_blocks_small_input() {

        assert_eq!(2858, 2858);
    }
}