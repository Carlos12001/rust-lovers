# Day 00003 – Sum of Even Digits

**Folder / Rust package:** day00003_sum_even_digits

## 🧠 Problem Statement

Write a program that reads a **positive integer** `n` from the user, and computes the **sum of all its even digits**.

If the input is not a valid positive integer, the program should handle the error gracefully.

## ✅ Requirements

- Read a number `n` from user input.
- Iterate through its digits.
- Add only the **even digits** (`0, 2, 4, 6, 8`) to a running total.
- Print the total sum of even digits.
- Input must be validated (i.e., reject non-digit input or negative numbers).

## 📥 Example Input

$$$bash
Enter a number: 246813
$$$

## 📤 Expected Output

$$$bash
Sum of even digits: 12
$$$

## Secure Tips to create the code

- Use `.chars()` and `.parse::<u32>()` to safely convert digits.
- Use `match` or `if digit % 2 == 0` to check even digits.
- Handle invalid input with `match` or `.parse().ok_or()`.

## What use C or C++

Use **Rust only** for this exercise.

