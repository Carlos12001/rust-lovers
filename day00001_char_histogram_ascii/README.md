## Day 00001 - Char Histogram (ASCII)

**Folder / Rust package:** `day00001_char_histogram_ascii`

### 🧠 Problem Statement

Write a program that reads a single line of text from standard input and counts how many times each letter (A–Z or a–z) appears. The program should treat uppercase and lowercase letters as the same and ignore all non-letter characters.

Only the letters that appear at least once should be printed, and each should be followed by a number of asterisks (`*`) representing its frequency.

---

### ✅ Requirements

- Input: a single line of text
- Case-insensitive (e.g., `A` and `a` are treated the same)
- Ignore non-alphabetic characters (no digits, punctuation, or `ñ`)
- Output should include:
  - One line per letter used
  - Format: `letter: *****`
  - Sorted alphabetically by letter

---

### 📥 Example Input

```bash
Hello, World!
```

### 📤 Expected Output

```bash
d: *
e: *
h: *
l: ***
o: **
r: *
w: *
```

### 🌟 Bonus Challenge

- Sort the output by frequency (descending) instead of alphabetical order.
- Display total character count and total unique letters used.
