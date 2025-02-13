use project_3::garbage_coll::{reference_counting, mark_and_sweep, stop_and_copy};
use project_3::types::{Memory, RefCountMem};

#[test]
fn public_reference_counting1() {
    assert_eq!(reference_counting("tests/rf_inputs/empty.txt"), RefCountMem {
        stack: vec![],
        heap: vec![(None, 0); 10]
    })
}

#[test]
fn public_reference_counting2() {
    assert_eq!(reference_counting("tests/rf_inputs/basic.txt"), RefCountMem {
        stack: vec![vec![2]],
        heap: vec![(Some(vec![5, 2]), 1), (None, 0), (Some(vec![0,3,4]), 2), (Some(vec![]), 1), (Some(vec![]), 1), 
        (Some(vec![]), 1), (None, 0), (None, 0), (None, 0), (None, 0)],
    })
}

#[test]
fn public_reference_counting3() {
    assert_eq!(reference_counting("tests/rf_inputs/complex1.txt"), RefCountMem {
        stack: vec![],
        heap: vec![(None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0)],
    })
}

#[test]
fn public_reference_counting4() {
    assert_eq!(reference_counting("tests/rf_inputs/complex2.txt"), RefCountMem {
        stack: vec![vec![1, 4, 6, 9], vec![3, 4], vec![7,5] ],
        heap: vec![(Some(vec![]), 1), (Some(vec![5, 3]), 1), (Some(vec![]), 2), (Some(vec![9, 2, 5]), 2),
                    (Some(vec![]), 2), (Some(vec![]), 3), (Some(vec![]), 1), (Some(vec![8, 2, 0]), 1), (Some(vec![]), 1), (Some(vec![]), 2)
        ],
    })
}

// continuation of the previous test (4), popping two programs off the stack
#[test]
fn public_reference_counting5() {
    assert_eq!(reference_counting("tests/rf_inputs/complex3.txt"), RefCountMem {
        stack: vec![vec![1, 4, 6, 9]],
        heap: vec![(None, 0), (Some(vec![5, 3]), 1), (Some(vec![]), 1), (Some(vec![9, 2, 5]), 1),
                    (Some(vec![]), 1), (Some(vec![]), 2), (Some(vec![]), 1), (None, 0), (None, 0), (Some(vec![]), 2)
        ],
    })
}

// continuation of the previous test, popping the last program off the stack
#[test]
fn public_reference_counting6() {
    assert_eq!(reference_counting("tests/rf_inputs/complex4.txt"), RefCountMem {
        stack: vec![],
        heap: vec![(None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0), (None, 0)],
    })
}


#[test]
fn public_mark_and_sweep1() {
    let input: Vec<Option<(String, Vec<u32>)>> = vec![Some(("A".to_string(), vec![])), Some(("B".to_string(), vec![])), Some(("C".to_string(), vec![])), Some(("D".to_string(), vec![]))];
    let expected = vec![None, Some(("B".to_string(), vec![])), None, Some(("D".to_string(), vec![]))];
    let mut mem = Memory {
        stack : vec![vec! [1,3]],
        heap: input,
    };
    mark_and_sweep(&mut mem);
    assert_eq!(expected, mem.heap); 
}


#[test]
fn public_mark_and_sweep2() {
    let input = vec! [Some(("A".to_string(), vec![1])), 
    Some(("B".to_string(), vec![3])), Some(("C".to_string(), vec![])), Some(("D".to_string(), vec![]))];

    let expected = vec! [Some(("A".to_string(), vec![1])), 
    Some(("B".to_string(), vec![3])), None, Some(("D".to_string(), vec![]))];

    let mut mem = Memory {
        stack : vec! [vec! [0]],
        heap: input,
    };
    mark_and_sweep(&mut mem);
    assert_eq!(expected, mem.heap); 
}


#[test]
fn public_mark_and_sweep3() {
    let input = vec! [Some(("A".to_string(), vec![1])), 
    Some(("B".to_string(), vec![0])), Some(("C".to_string(), vec![])), Some(("D".to_string(), vec![]))];
    let expected = vec! [Some(("A".to_string(), vec![1])), 
    Some(("B".to_string(), vec![0])), None, None];

    let mut mem = Memory {
        stack : vec! [vec! [0]],
        heap: input,
    };
    mark_and_sweep(&mut mem);
    assert_eq!(expected, mem.heap); 
}

#[test]
fn public_mark_and_sweep4() {
    let input = vec! [Some(("A".to_string(), vec![1])), 
    Some(("B".to_string(), vec![0])), Some(("C".to_string(), vec![])), Some(("D".to_string(), vec![]))];

    let expected = vec! [None, None, None, None];

    let mut mem = Memory {
        stack : vec! [vec! []],
        heap:  input,
    };
    mark_and_sweep(&mut mem);
    assert_eq!(expected, mem.heap); 
}

#[test]
fn public_mark_and_sweep5() {
    let input = vec! [Some(("A".to_string(), vec![1])), 
    Some(("B".to_string(), vec![0])), Some(("C".to_string(), vec![])), Some(("D".to_string(), vec![0]))];

    let expected = vec! [Some(("A".to_string(), vec![1])), 
    Some(("B".to_string(), vec![0])), None, Some(("D".to_string(), vec![0]))];
    let mut mem = Memory {
        stack : vec! [vec! [3]],
        heap: input,
    };
    mark_and_sweep(&mut mem);
    assert_eq!(expected, mem.heap);  
}


#[test]
fn public_stop_copy1() {

    let mut mem = Memory {
        stack: vec![vec![6]], 
        heap : vec! [Some(("A".to_string(), vec![1])), Some(("B".to_string(), vec![0])), 
        Some(("C".to_string(), vec![])), Some(("D".to_string(), vec![2])), //dead 

        Some(("E".to_string(), vec![])), None, Some(("F".to_string(), vec![4])), None] // alive
    };
    
    let expected = Memory {
        stack: vec![ vec![1]], 
        heap : vec! [Some(("E".to_string(), vec![])), Some(("F".to_string(), vec![0])), None, None, // alive 
        
        Some(("E".to_string(), vec![])), None, Some(("F".to_string(), vec![4])), None] // dead
    };

    let expected2 = Memory {
        stack: vec![ vec![0]], 
        heap : vec! [Some(("F".to_string(), vec![1])), Some(("E".to_string(), vec![])), None, None, // alive 
        
        Some(("E".to_string(), vec![])), None, Some(("F".to_string(), vec![4])), None] // dead
    };
    
    stop_and_copy(&mut mem, 1);
    assert!(mem == expected || mem == expected2); // either permutation is fine

}

#[test]
fn public_stop_copy2() {

    let mut mem = Memory {
        stack: vec![vec![3]], 
        heap : vec![
                    Some(("A".to_string(), vec![2])), Some(("B".to_string(), vec![2])), Some(("C".to_string(), vec![4])),
                     Some(("D".to_string(), vec![])), Some(("E".to_string(), vec![1])), // alive

                    Some(("F".to_string(), vec![6,7,8])), Some(("G".to_string(), vec![9])), 
                    Some(("H".to_string(), vec![])), Some(("I".to_string(), vec![9])), Some(("J".to_string(), vec![7])) // dead
            ]
    };
    
    let expected = Memory {
        stack: vec![vec![5]], 
        heap: vec![
                    Some(("A".to_string(), vec![2])), Some(("B".to_string(), vec![2])), 
                    Some(("C".to_string(), vec![4])), Some(("D".to_string(), vec![])), 
                    Some(("E".to_string(), vec![1])), // dead

                    Some(("D".to_string(), vec![])), None, None, None, None // alive
            ] 
    };
    
    stop_and_copy(&mut mem, 0);
    assert_eq!(mem, expected);
}

