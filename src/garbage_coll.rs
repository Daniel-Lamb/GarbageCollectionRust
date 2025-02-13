use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

use crate::types::{Memory, RefCountMem};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}








//start of functions I need to implement

pub fn reference_counting(filename: &str) -> RefCountMem {
    // Initialize the stack and heap
    let mut stack: Vec<Vec<u32>> = Vec::new();
    let mut heap: Vec<(Option<Vec<u32>>, u32)> = vec![(None, 0); 10];

    // Helper functions
    fn increment_ref(heap: &mut Vec<(Option<Vec<u32>>, u32)>, idx: usize) {
        if idx >= heap.len() {
            return;
        }
        heap[idx].1 += 1;
    }

    fn decrement_ref(heap: &mut Vec<(Option<Vec<u32>>, u32)>, idx: usize) {
        if idx >= heap.len() {
            return;
        }
        if heap[idx].1 > 0 {
            heap[idx].1 -= 1;
            if heap[idx].1 == 0 {
                // Deallocate heap[idx] and recursively decrement its references
                if let Some(refs) = heap[idx].0.take() {
                    for &r in &refs {
                        decrement_ref(heap, r as usize);
                    }
                }
            }
        }
    }

    fn adjust_refcount(heap: &mut Vec<(Option<Vec<u32>>, u32)>, idx: usize, delta: i32) {
        if idx >= heap.len() {
            return;
        }
        let count = &mut heap[idx].1;
        if delta > 0 {
            *count += delta as u32;
        } else {
            let decrement = (-delta) as u32;
            if *count >= decrement {
                *count -= decrement;
            } else {
                *count = 0;
            }
        }
    }

    // Regular expressions
    let heap_ref_re = Regex::new(r"Ref Heap (([0-9]+) ?)*").unwrap();
    let stack_ref_re = Regex::new(r"Ref Stack (([0-9]+) ?)*").unwrap();
    let num_lst_re = Regex::new(r"([0-9]+) ?").unwrap();
    let pop_re = Regex::new(r"Pop").unwrap();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            if heap_ref_re.is_match(&line) {
                let mut numbers: Vec<u32> = vec![];
                num_lst_re.captures_iter(&line).for_each(|f| {
                    numbers.push(f.get(1).unwrap().as_str().parse::<u32>().unwrap());
                });

                if numbers.is_empty() {
                    continue;
                }
                let idx = numbers[0] as usize;
                let refs = numbers[1..].to_vec();

                if idx >= heap.len() {
                    continue;
                }

                // Adjust ref counts of old references
                if let Some(old_refs) = heap[idx].0.clone() {
                    for &r in &old_refs {
                        adjust_refcount(&mut heap, r as usize, -1);
                    }
                }

                // Update heap[idx] with new references
                heap[idx].0 = Some(refs.clone());

                // Adjust ref counts of new references
                for &r in &refs {
                    adjust_refcount(&mut heap, r as usize, 1);
                }

            } else if stack_ref_re.is_match(&line) {
                let mut numbers: Vec<u32> = vec![];

                num_lst_re.captures_iter(&line).for_each(|f| {
                    numbers.push(f.get(1).unwrap().as_str().parse::<u32>().unwrap());
                });

                // Handle Ref Stack
                stack.push(numbers.clone());

                // Increment reference counts of the numbers
                for &r in &numbers {
                    if r as usize >= heap.len() {
                        continue;
                    }
                    increment_ref(&mut heap, r as usize);
                }

            } else if pop_re.is_match(&line) {
                // Handle Pop
                if let Some(frame) = stack.pop() {
                    // Decrement reference counts of the numbers
                    for &r in &frame {
                        if r as usize >= heap.len() {
                            continue;
                        }
                        decrement_ref(&mut heap, r as usize);
                    }
                }
                // If the stack is empty, Pop has no effect

            } else {
                panic!("no matches");
            }
        }
    };

    // After processing all lines, ensure that allocated entries with non-zero reference counts have data
    for idx in 0..heap.len() {
        if heap[idx].0.is_none() && heap[idx].1 > 0 {
            heap[idx].0 = Some(vec![]);
        }
    }

    // Return the RefCountMem struct
    RefCountMem {
        stack,
        heap,
    }
}
























// suggested helper function. You may modify parameters as you wish.
// Takes in some form of stack and heap and returns all indicies in heap
// that can be reached.
pub fn reachable(stack: &Vec<Vec<u32>>, heap: &Vec<Option<(String, Vec<u32>)>>) -> Vec<u32> {
    let mut visited = HashMap::new(); // visited
    let mut worklist = Vec::new();    // Worklist

    // werking it w/ worklist
    for frame in stack {
        for &idx in frame {
            if !visited.contains_key(&idx) {
                visited.insert(idx, true);
                worklist.push(idx);
            }
        }
    }

    while let Some(idx) = worklist.pop() {
        if let Some(Some((_name, refs))) = heap.get(idx as usize) {
            for &ref_idx in refs {
                if !visited.contains_key(&ref_idx) {
                    visited.insert(ref_idx, true);
                    worklist.push(ref_idx);
                }
            }
        }
    }

    visited.keys().cloned().collect()
}







pub fn mark_and_sweep(mem: &mut Memory) -> () {
    // get reachables
    let reachable_indices = reachable(&mem.stack, &mem.heap);

    // lookup in a hashmap
    let mut reachable_map = HashMap::new();
    for idx in reachable_indices {
        reachable_map.insert(idx, true);
    }

    for idx in 0..mem.heap.len() {
        if !reachable_map.contains_key(&(idx as u32)) {
            mem.heap[idx] = None; // deallocate unreachables
        }
    }
}









// alive says which half is CURRENTLY alive. You must copy to the other half
// 0 for left side currently in use, 1 for right side currently in use
pub fn stop_and_copy(mem: &mut Memory, alive: u32) -> () {
    let heap_size = mem.heap.len();
    if heap_size % 2 != 0 {
        return;
    }
    let half_size = heap_size / 2;

    let (from_start, to_start) = if alive == 0 {
        (0, half_size)
    } else {
        (half_size, 0)
    };

    let mut new_heap = vec![None; heap_size];
    let mut index_map = HashMap::new();
    let mut next_free = to_start;

    let mut worklist = Vec::new();

    // start from roots
    for frame in &mut mem.stack {
        for idx in frame.iter_mut() {
            let old_idx = *idx as usize;
            if old_idx < from_start || old_idx >= from_start + half_size {
                continue;
            }
            if let Some(&new_idx) = index_map.get(&old_idx) {
                *idx = new_idx as u32;
            } else {
                // cpy
                if next_free >= to_start + half_size {
                    continue;
                }
                new_heap[next_free] = mem.heap[old_idx].clone();
                index_map.insert(old_idx, next_free);
                *idx = next_free as u32;
                worklist.push((old_idx, next_free));
                next_free += 1;
            }
        }
    }

    // process worklist
    while let Some((_old_idx, new_idx)) = worklist.pop() {
        let refs_clone;
        {
            let entry = &mut new_heap[new_idx];
            if let Some((_name, refs)) = entry {
                refs_clone = refs.clone();
            } else {
                continue;
            }
        }
        // process refs_clone
        let mut updated_refs = Vec::new();
        for idx in refs_clone {
            let old_ref_idx = idx as usize;
            if old_ref_idx < from_start || old_ref_idx >= from_start + half_size {
                continue;
            }
            let new_ref_idx = if let Some(&new_ref_idx) = index_map.get(&old_ref_idx) {
                new_ref_idx
            } else {
                // cpy refernced
                if next_free >= to_start + half_size {
                    continue;
                }
                new_heap[next_free] = mem.heap[old_ref_idx].clone();
                index_map.insert(old_ref_idx, next_free);
                worklist.push((old_ref_idx, next_free));
                let new_ref_idx = next_free;
                next_free += 1;
                new_ref_idx
            };
            updated_refs.push(new_ref_idx as u32);
        }

        if let Some((_name, refs)) = &mut new_heap[new_idx] {
            *refs = updated_refs;
        }
    }

    // replace alive half wit new half
    for idx in 0..half_size {
        let target_idx = to_start + idx;
        mem.heap[target_idx] = new_heap[target_idx].take();
    }
}