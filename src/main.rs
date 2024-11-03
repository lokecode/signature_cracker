extern crate tiny_keccak;

use std::process;
use tiny_keccak::{Hasher, Keccak};

fn main() {
    brute_force_signature();

    let mut guess: Vec<String> = Vec::new();
    guess.push(String::from("address[3]"));
    guess.push(String::from("uint256"));

    check_guess(guess)
}

//ee63c1e5
//swapUniV3((bytes26,bytes31[]),uint8,int248[])

const TYPES: [&str; 48] = [
    "address",
    "bool",
    "string",
    "tuple",
    "bytes",
    "bytes1",
    "bytes2",
    "bytes3",
    "bytes4",
    "bytes5",
    "bytes6",
    "bytes7",
    "bytes8",
    "bytes9",
    "bytes10",
    "bytes11",
    "bytes12",
    "bytes13",
    "bytes14",
    "bytes15",
    "bytes16",
    "bytes20",
    "bytes25",
    "bytes26",
    "bytes27",
    "bytes28",
    "bytes29",
    "bytes30",
    "bytes31",
    "bytes32",
    "uint8",
    "uint16",
    "uint24",
    "uint32",
    "uint160",
    "uint168",
    "uint240",
    "uint248",
    "uint256",
    "int8",
    "int16",
    "int24",
    "int32",
    "int160",
    "int168",
    "int240",
    "int248",
    "int256"
];

fn brute_force_signature() {
    const DEPTH: i8 = 3;
    let mut guess: Vec<String> = Vec::new();
    let all_types: Vec<String> = get_array_types_with_num();
    check_all_combinations(0, &mut guess, all_types);
    fn check_all_combinations(index: i8, guess: &mut Vec<String>, all_types: Vec<String>) {
        for sol_type in &all_types {
            if guess.len() > index as usize {
                guess[index as usize] = sol_type.to_string();
            } else {
                guess.push(sol_type.to_string());
            }
            check_guess(guess.clone());

            if index != DEPTH {
                check_all_combinations(index + 1, guess, all_types.clone());
            }
        }
    }

    fn get_array_types() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for sol_type in &TYPES {
            output.push(sol_type.to_string());
            output.push(format!("{}{}", sol_type, "[]"))
        }
        output
    }
    fn get_array_types_with_num() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for sol_type in &TYPES {
            output.push(sol_type.to_string());
            output.push(format!("{}{}", sol_type, "[]"));
            for index in 1..5 {
                output.push(format!("{}{}{}{}", sol_type, "[",index,"]"));
            }
        }
        output
    }
}

fn check_guess(guess: Vec<String>) {
    is_guess_correct(guess.clone());
    check_tuple_combinations(guess.clone());
}

fn is_guess_correct(guess: Vec<String>) {
    std::thread::spawn(move || {
        let function = format!("swapUniV3({}{}", guess.join(","), ")");
        let signature = get_selector(function.as_str());

        //ee63c1e5
        if signature.contains("ee63c1e") {
            println!("{}", signature);
            println!("{}", function)
        }
        if signature == "ee63c1e5" {
            process::exit(0);
        }
    });
}

fn check_tuple_combinations(guess: Vec<String>) {
    for i1 in 0..guess.len() {
        for i2 in 0..(guess.len()-i1) {
            let mut new_guess: Vec<String> = guess.clone();
            let end = (guess.len()-i2)-1;
            let item = guess[i1].clone();

            new_guess[i1] = format!("{}{}", "(",item);
            new_guess[end] = format!("{}{}", new_guess[end],")");

            is_guess_correct(new_guess);
        }
    }
}


fn get_selector(function_signature: &str) -> String {
    let signature_bytes = function_signature.as_bytes();

    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32]; // Keccak256 hash output is 32 bytes

    hasher.update(signature_bytes);
    hasher.finalize(&mut output);

    let selector = &output[..4];

    hex::encode(selector)
}
