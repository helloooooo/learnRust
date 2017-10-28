//stdクレートのfsモジュールにあるfile型をインポート。以後はFileで参照可能
use std::fs::File;

//同じモジュールから複数インポートする場合は{}でまとめて指定できる
use std::io::{BufReader, BufRead};

//モジュール全体のインポートもできる
use std::env;

extern crate regex;

use regex::Regex;

fn usage() {
    println!("rsgrep PATTERN FILENAME");
}

fn main() {
    let pattern = match env::args().nth(1) {
        Some(pattern) => pattern,
        None => {
            usage();
            return;
        }
    };

    let reg = match Regex::new(&pattern) {
        Ok(reg) => reg,
        Err(e) => {
            println!("invalid regexp {} :{} ", pattern, e);
            return;
        }
    };

    let filename = match env::args().nth(2) {
        Some(filename) => filename,
        None => {
            usage();
            return;
        }
    };

    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => {
            println!("An error occured while opening file {}:{}", filename, e);
            return;
        }
    };
    let input = BufReader::new(file);

    for line in input.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("An error occured while reading a line {}", e);
                return;
            }
        };

        if reg.is_match(&line) {
            println!("{}", line);
        }
    }
}
