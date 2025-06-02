fn unique_paths_in_grid(grid: &Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();

    if grid[0][0] == 1 || grid[n - 1][m - 1] == 1 {
        return 0;
    }

    let mut dp = vec![vec![0; m]; n];
    dp[0][0] = 1;

    for j in 1..m {
        dp[0][j] = if grid[0][j] == 0 && dp[0][j - 1] == 1 {
            1
        } else {
            0
        };
    }

    for i in 1..n {
        dp[i][0] = if grid[i][0] == 0 && dp[i - 1][0] == 1 {
            1
        } else {
            0
        };
    }

    for i in 1..n {
        for j in 1..m {
            if grid[i][j] == 0 {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
    }

    dp[n - 1][m - 1]
}

fn print_matrix(grid: &Vec<Vec<i32>>) {
    println!("Grid:\n");
    for row in grid {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn parse_matrix_arg(arg: &str) -> Vec<Vec<i32>> {
    arg.split(';')
        .map(|row| {
            row.split(',')
                .map(|v| v.trim().parse::<i32>().expect("Invalid integer"))
                .collect()
        })
        .collect()
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run              # Use default 3x3 matrix");
    println!("  cargo run -- -m 0,0,0;0,1,0;0,0,0   # Custom matrix");
    println!("  cargo run -- -h        # Show help");
}

fn main() {
    println!("============================================");
    println!("  Unique Paths Grid Solver (CLI Program)   ");
    println!("============================================");
    println!("This program calculates how many unique paths");
    println!("exist in a 2D grid from top-left to bottom-right,");
    println!("moving only right and down, avoiding blocked cells (1).\n");

    let args: Vec<String> = std::env::args().collect();
    let grid: Vec<Vec<i32>>;

    if args.len() == 1 {
        println!("Tip: You can pass a matrix using -m or see help with -h");
        println!("No arguments provided. Using default 3x3 matrix:");
        grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    } else if args.len() == 2 && args[1] == "-h" {
        print_help();
        return;
    } else if args.len() == 3 && args[1] == "-m" {
        match std::panic::catch_unwind(|| parse_matrix_arg(&args[2])) {
            Ok(parsed) => grid = parsed,
            Err(_) => {
                eprintln!(
                    "Error parsing matrix. Use format: 0,0,0;0,1,0;0,0,0"
                );
                return;
            }
        }
    } else {
        eprintln!("Invalid arguments. Use -h for help.");
        return;
    }

    print_matrix(&grid);
    let result = unique_paths_in_grid(&grid);
    println!("\n\nResult:\n{}", result);
}
