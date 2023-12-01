//use std::env;
use std::fs;

fn print_arr(arr: &Vec<usize>) {
    for elem in arr {
        print!("{elem} ");
    }
    print!("\n");
}
fn read_file(file_path: &str) -> String {
    //let s = String::new();
    let s = fs::read_to_string(file_path).expect("failed");
    s
}

fn solve1() {
    // println!("Hello, world!");
    let file_path = "2.in";
    let data = read_file(file_path);
    // println!("{data}");
    let lines = data.split("\n");
    let mut total_sum = 0;
    for line in lines {
        // let mut nums : Vec<i32> = Vec::new();
        let mut num_chrs: Vec<char> = Vec::new();
        println!("line: {line}");
        for c in line.chars() {
            if c == '0' {
                //nums.push(0);
                num_chrs.push('0');
            } else if c == '1' {
                num_chrs.push('1');
                //nums.push(1);
            } else if c == '2' {
                num_chrs.push('2');
                //nums.push(2);
            } else if c == '3' {
                num_chrs.push('3');
                //nums.push(3);
            } else if c == '4' {
                num_chrs.push('4');
                //nums.push(4);
            } else if c == '5' {
                num_chrs.push('5');
                //nums.push(5);
            } else if c == '6' {
                num_chrs.push('6');
                //nums.push(6);
            } else if c == '7' {
                num_chrs.push('7');
                //nums.push(7);
            } else if c == '8' {
                num_chrs.push('8');
                //nums.push(8);
            } else if c == '9' {
                num_chrs.push('9');
                //nums.push(9);
            }
        }
        //let first = nums[0];
        //let last = nums[nums.len()-1];
        //let line_sum = first + last;
        let mut line_num_str = String::new();
        line_num_str.push(num_chrs[0]);
        line_num_str.push(num_chrs[num_chrs.len() - 1]);

        let line_num = line_num_str.parse::<i32>().unwrap();
        total_sum = total_sum + line_num;
        //println!("{} nums, line sum = {}", num_chrs.len(), line_num );
    }
    println!("And the answer is {total_sum} ");
}
fn line2sum(line: &str) -> usize {
    let nums_as_chrs = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let nums_as_strs = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    //let nums_as_ints = [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let n_nums = 10;
    let mut i = 0;
    let n = line.len();
    let mut line_nums: Vec<usize> = Vec::new();
    while i < n {
        let mut found_chr = false;
        for j in 0..10 {
            //println!("{j}");
            if line.chars().nth(i).unwrap() == nums_as_chrs[j] {
                found_chr = true;
                line_nums.push(j);
                break;
            }
        }
        if !found_chr {
            for j in 0..10 {
                let mut found_str = true;

                let str = nums_as_strs[j];
                let strlen = str.len();
                for si in 0..strlen {
                    if i + si > line.len() - 1 {
                        found_str = false;
                        break;
                    }
                    //println!("{} {} {} {} {}", si, str, i+si, line, li);
                    if str.chars().nth(si).unwrap() != line.chars().nth(i + si).unwrap() {
                        found_str = false;
                        //line_nums.push(si);

                        break;
                    }
                }
                if found_str {
                    //i+=strlen-1;
                    line_nums.push(j);
                    break;
                }
            }
        }
        i += 1;
    }
    // print!("{line} ");
    // print_arr(&line_nums);
    line_nums[0] * 10 + line_nums[line_nums.len() - 1]
}
fn solve2() {
    let file_path = "2.in";
    let data = read_file(file_path);
    // println!("{data}");
    let lines = data.split("\n");
    let mut total_sum = 0;
    for line in lines {
        // let mut nums : Vec<i32> = Vec::new();
        //let mut num_chrs : Vec<char> = Vec::new();
        let line_sum = line2sum(&line);
        total_sum += line_sum;
    }
    println!("And the answer is: {total_sum}");
}

fn main() {
    solve2();
}
