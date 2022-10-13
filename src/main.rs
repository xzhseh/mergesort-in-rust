use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

fn mergesort(merge_vec: &mut Vec<i32>, begin: usize, end: usize) {
    if begin >= end {
        return;
    }

    let middle = begin + (end - begin) / 2; // Prevent the possible cases of overflow

    mergesort(merge_vec, begin, middle);
    mergesort(merge_vec, middle + 1, end);

    merge_the_vector(merge_vec, begin, middle, end);
}

fn merge_the_vector(the_vec: &mut Vec<i32>, begin: usize, middle: usize, end: usize) {
    let mut tmp_first_vec: Vec<i32> = vec![0; middle - begin + 1];
    let mut tmp_second_vec: Vec<i32> = vec![0; end - middle];

    tmp_first_vec.clone_from_slice(&the_vec[begin..=middle]);
    tmp_second_vec.clone_from_slice(&the_vec[(middle + 1)..=end]);

    println!("the first vec: {:?}", tmp_first_vec);
    println!("the second vec: {:?}", tmp_second_vec);

    let mut ptr_one: usize = 0;
    let mut ptr_two: usize = 0;
    let mut ptr_for_vec: usize = begin;

    while ptr_one < (middle - begin + 1) && ptr_two < (end - middle) {
        if tmp_first_vec[ptr_one] < tmp_second_vec[ptr_two] {
            the_vec[ptr_for_vec] = tmp_first_vec[ptr_one];
            ptr_one += 1;
        } else {
            the_vec[ptr_for_vec] = tmp_second_vec[ptr_two];
            ptr_two += 1;
        }
        ptr_for_vec += 1;
    }

    // the first vector reach the size, the just append the elements in the other to the_vec
    // Vice versa
    if ptr_one == (middle - begin + 1) { 
        while ptr_two < (end - middle) {
            the_vec[ptr_for_vec] = tmp_second_vec[ptr_two];
            ptr_two += 1;
            ptr_for_vec += 1;
        }
    } else {
        while ptr_one < (middle - begin + 1) {
            the_vec[ptr_for_vec] = tmp_first_vec[ptr_one];
            ptr_one += 1;
            ptr_for_vec += 1;
        }
    }
    println!("After merge: {:?}", the_vec);
}

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let mut buffer = String::new();
    let _size = File::open(filename)?.read_to_string(&mut buffer)?;
    Ok(buffer.split('\n').filter(|x| *x != "").map(|x| x.to_string()).collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffer = read_file_lines(&"mergesort.in".to_string())?;
    let len = buffer[0].parse::<usize>()?;
    let mut merge_vec = buffer[1].split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    mergesort(&mut merge_vec, 0, len - 1);
    println!("The final result {:?}", merge_vec);
    let final_result = merge_vec.iter().map(|x| {
        let mut tmp = x.to_string();
        tmp.push(' ');
        tmp
    }).collect::<String>();
    let final_result = final_result.trim();
    let mut file = File::create("mergesort.out").expect("Error in creating mergesort.out!");
    file.write(&final_result.as_bytes()).expect("Error in writing to mergesort.out!");
    Ok(())
}
