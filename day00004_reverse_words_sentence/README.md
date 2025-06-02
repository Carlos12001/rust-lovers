# Day 00004 – Reverse Words in a Sentence

**Folder / Rust package:** day00004_reverse_words_sentence

## 🧠 Problem Statement

Write a Rust program that reads a full sentence (a string) and **prints the sentence with the words in reverse order**, preserving the original words exactly as they are.

- Only whitespace should separate words.
- Punctuation and casing should remain unchanged.

## ✅ Requirements

- Accept the sentence as a command-line argument or ask the user for input.
- Reverse the **word order** (not the characters).
- Print the result as a single line.

## 📥 Example Input

$$$bash
Rust is awesome!
$$$

## 📤 Expected Output

$$$bash
awesome! is Rust
$$$

## Secure Tips to create the code

- Use `.split_whitespace()` to split the input safely.
- Use `.rev()` on an iterator to reverse the word order.
- Join the reversed words using `.collect::<Vec<_>>().join(" ")`.
- Trim the input before processing.

## What use C or C++

Use **Rust only** for this exercise.

