#![allow(unused)]

use std::io;
use std::collections::HashMap;
static MAXIMUM_OPPORTUNITY: i32 = 6;
static LENGTH_OF_WORD: i32 = 5;

fn main() {
    println!("Please type your 5 letters word below!");
    let mut target_str = String::new();
    let mut map : HashMap<char, u8> = HashMap::new();
    
    io::stdin().read_line(&mut target_str);
    let trimmed_target_str = target_str.trim();

    let target_str_len = target_str.len();
    println!("target_str_len : {}", target_str_len - 1);

    if target_str_len != 6 { // consider carrage return for a target string
        println!("error");
    }

    let mut n = 1;
    let mut candidate = String::new();

    for ch in trimmed_target_str.chars() {
        let counter = map.entry(ch).or_insert(0);
        *counter += 1;
    }
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    // for value in target_str.chars() {
    //     if map.contains_key(&value) {
    //         *map.get_mut(&value).unwrap() += 1;
    //     }
    //     else {
    //         map.insert(value, 1);
    //     }
    // }

    // for val in map.values() {
    //     println!("{}", val);
    // }

    while n <= MAXIMUM_OPPORTUNITY {
        io::stdin().read_line(&mut candidate);
        let trimmed_candidate = candidate.trim();
        // candidate.replace("\n", "");
        if trimmed_candidate.eq(&target_str) {
            println!("success!");
            break;
        }
        else {
            notice_hint(&mut map, &trimmed_target_str, &trimmed_candidate);
        }
        n += 1;
    }
}

fn notice_hint(h: &mut HashMap<char, u8>, target: &str, candidate: &str) -> String {
    /*
        Input :
        Output :    5 letters word filled with 
                    B(Black, wrong character), 
                    Y(Yellow, wrong position but right charater), 
                    G(Green, right position and right charater)
        Description :  
    */

    let target_array: Vec<char> = target.chars().collect();
    let candidate_array: Vec<char> = candidate.chars().collect();
    // let mut idx = 0;
    let mut result = String::new();
    let mut idx = 0;

    for value in candidate.chars() {
        if target_array[idx] == candidate_array[idx] {
            result.push('G');
        }
        else if h.contains_key(&value) && *h.entry(value).or_insert(0) > 1 {
            // for ch in prac.chars() {
            //     let counter = h.entry(ch).or_insert(0);
            //     *counter += 1;
            // }
        
            result.push('Y');
        }
        else {
            result.push('B');
        }
        idx += 1;
    }
    println!("string : {}", result);
    return result; 
}
