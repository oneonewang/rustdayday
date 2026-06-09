// Exercise 07 · 集合
// 任务：见 README.md

use std::collections::HashMap;

fn filter_positive(v: Vec<i32>) -> Vec<i32> { todo!() }
fn sort_desc(v: &mut Vec<i32>) { todo!() }
fn dedup_sorted(v: &mut Vec<i32>) { todo!() }

fn reverse_words(s: &str) -> String { todo!() }
fn is_palindrome(s: &str) -> bool { todo!() }

fn char_count(s: &str) -> HashMap<char, usize> { todo!() }
fn top_k_words(text: &str, k: usize) -> Vec<(String, usize)> { todo!() }

fn wc(text: &str) -> (usize, usize, usize) { todo!() }

struct WordIndex { map: HashMap<String, Vec<usize>> }

impl WordIndex {
    fn from_text(text: &str) -> Self { todo!() }
    fn positions(&self, word: &str) -> Option<&Vec<usize>> { todo!() }
    fn unique_words(&self) -> usize { todo!() }
}

fn main() {
    // TODO
}
