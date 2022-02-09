#![allow(unused)]

use std::io;
use std::collections::HashMap;
static MAXIMUM_OPPORTUNITY: i32 = 6;
static LENGTH_OF_WORD: usize = 5;
const RANDOM_STRING: &'static str = "query";

mod wordle {
    pub struct Candidate {
        pub hint_string: String,
    }
    impl Candidate {
        pub fn init(hint_string: &str) -> Candidate {
            Candidate {
                hint_string: String::from(hint_string),
            }
        }
    }
}

mod word_generator {
    pub struct Generate {
        pub target: String,
    }
    impl Generate {
        pub fn init(target: &str) -> Generate {
            Generate {
                target: String::from(target),
            }
        }
    }
}

fn main() {

    let mut target_str = String::new();
    let mut map : HashMap<char, u8> = HashMap::new();
    let mut n = 1;
    let mut candidate = String::new();
    let mut answer = String::new();
    let mut trimmed_target_str;

    println!("1 : Get random target string | 2 : Set your own target string");
    // type answer 
    io::stdin().read_line(&mut answer);
    let mut trimmed_answer = answer.trim();
    let my_int = trimmed_answer.parse::<usize>().unwrap();

    match my_int {
        1 => println!("1 is chosen!"),
        2 => println!("2 is chosen!"),
        _ => println!("no answer"),
    }

    if my_int == 1 {
        let mut word_gen = word_generator::Generate::init(RANDOM_STRING);
        trimmed_target_str = word_gen.target;
        println!("{} ||", trimmed_target_str);
    }
    else if my_int == 2 {
        println!("Please type your 5 letters word below!");
    
        // type target string
        io::stdin().read_line(&mut target_str);
        trimmed_target_str = target_str.trim().to_string();
    }
    else {
        println!("no interest!");
        return;
    }

    // consider carrage return for a target string
    while !is_length_of_word_five(&trimmed_target_str) {
        
        target_str.clear();
       
        // re-enter target string
        io::stdin().read_line(&mut target_str);
        trimmed_target_str = target_str.trim().to_string();  

    }

    // init hashmap based on target string
    init_hashmap(&mut map, &trimmed_target_str);


    while n <= MAXIMUM_OPPORTUNITY {
        
        println!("--- Phase {} ---", n);

        // type candidate string
        io::stdin().read_line(&mut candidate);
        let trimmed_candidate = candidate.trim();

        if trimmed_candidate.eq(&trimmed_target_str) {
            println!("success!");
            break;
        }
        else {
            notice_hint(&mut map, &trimmed_target_str, &trimmed_candidate);
            candidate.clear();
        }

        n += 1;
    }
}

fn is_length_of_word_five(s: &str) -> bool {
    /* 
        Input : target string
        Output : true / false
        Description : Check whether the length of target string is five
    */

    if s.len() == LENGTH_OF_WORD {
        println!("length check completed!");
        true
    }
    else {
        println!("retry! the number of word is 5");
        false
    }
}

fn init_hashmap(h: &mut HashMap<char, u8>, target: &str) {
    /* 
        Input : hashmap, target string
        Output : x
        Description : By checking each character of word, this function counts alphabet
    */

    for ch in target.chars() {
        let counter = h.entry(ch).or_insert(0);
        *counter += 1;
    }
}

fn notice_hint(h: &mut HashMap<char, u8>, target: &str, candidate: &str) {
    /*
        Input : hashmap, target string, candidate string
        Output : x
        Description : whenever a user type candidate string, this function notice hint.is_length_of_word_five
                (
                    5 letters word filled with 
                    B(Black, wrong character), 
                    Y(Yellow, wrong position but right charater), 
                    G(Green, right position and right charater)
                )
    */

    let target_array: Vec<char> = target.chars().collect();
    let candidate_array: Vec<char> = candidate.chars().collect();
    let mut result = wordle::Candidate::init("Hint is : ");
    let mut idx = 0;

    for value in candidate.chars() {
        // println!("value : {}", value);
        if target_array[idx] == candidate_array[idx] {
            // result.push('G');
            result.hint_string.push('G');
            let counter = h.entry(value).and_modify(|e| { *e -= 1 });
            // *counter -= 1;
        }
        else if h.contains_key(&value) && *h.entry(value).or_insert(0) > 1 {
            result.hint_string.push('Y');
        }
        else {
            result.hint_string.push('B');
        }
        idx += 1;
    }

    println!("{}", result.hint_string);
}
