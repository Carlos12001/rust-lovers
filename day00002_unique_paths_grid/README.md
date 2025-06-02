# Day 00002 – Unique Paths in a Grid

**Folder / Rust package:** day00002_unique_paths_grid

## 🧠 Problem Statement

You are given a 2D list `grid[][]` of size `n x m` consisting of values `0` and `1`.

- A value of `0` means that you can enter that cell.
- A value of `1` implies that entry to that cell is **not allowed**.

You start at the upper-left corner of the grid `(1, 1)` and must reach the bottom-right corner `(n, m)`, **moving only to the right or down** at each step.

Your task is to calculate the **total number of unique paths** to reach the target.

> Note: The start `(1, 1)` and end `(n, m)` cells **can also be `1`**, in which case there is **no valid path**.
> It is guaranteed that the total number of paths fits in a **32-bit signed integer**.

## ✅ Requirements

- Only move **right** or **down**.
- Count all **valid paths** from `(0, 0)` to `(n-1, m-1)`.
- Return 0 if the start or end cell is blocked.
- Use efficient memory to handle up to 1 million cells.

## 📥 Example Input

```
bash
n = 3
m = 3
grid = [[0, 0, 0],
        [0, 1, 0],
        [0, 0, 0]]
```

## 📤 Expected Output

```
bash
2
```

## 📥 Example Input

```
bash
n = 1
m = 3
grid = [[1, 0, 1]]
```

## 📤 Expected Output

```
bash
0
```

## Secure Tips to create the code

- Use **C++ STL vectors** (`std::vector<std::vector<int>>`) to simplify dynamic memory management.
- Initialize a 2D `dp` table: `dp[i][j]` = number of ways to reach cell `(i, j)`.
- If `grid[i][j] == 1`, set `dp[i][j] = 0` because it's blocked.
- Base case: if `grid[0][0] == 1`, return 0 early.
- Use modulo logic only if the problem required overflow protection – here it does not.
- Optional: Optimize space to `O(m)` using a single row vector if needed.

## What use C or C++

Use **C++** for this exercise.
