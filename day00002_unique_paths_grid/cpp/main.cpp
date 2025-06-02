#include <iostream>
#include <sstream>
#include <string>
#include <vector>
using namespace std;

int unique_paths_in_grid(const vector<vector<int>>& grid) {
  const size_t n = grid.size();
  const size_t m = grid[0].size();

  if (n == 0 || m == 0) {
    cerr << "Error: grid must be initialized.\n";
    return -1;
  }

  if (grid[0][0] == 1 || grid[n - 1][m - 1] == 1) {
    return 0;
  }

  // dp[i][j] represents the number of ways to reach cell (i, j)
  vector<vector<int>> dp(n, vector<int>(m, 0));
  dp[0][0] = 1;  // Start cell has one path to itself

  // First row: can only come from the left
  for (size_t j = 1; j < m; ++j)
    dp[0][j] = (grid[0][j] == 0 && dp[0][j - 1] == 1) ? 1 : 0;

  // First column: can only come from above
  for (size_t i = 1; i < n; ++i)
    dp[i][0] = (grid[i][0] == 0 && dp[i - 1][0] == 1) ? 1 : 0;

  // General case: cell (i,j) can be reached from top or left
  for (size_t i = 1; i < n; ++i) {
    for (size_t j = 1; j < m; ++j) {
      if (grid[i][j] == 0) dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
    }
  }

  return dp[n - 1][m - 1];
}

void print_help() {
  cout << "Usage:\n"
       << "  ./program               Run with default 3x3 matrix.\n"
       << "  ./program -m 0,0,0;0,1,0;0,0,0   Run with custom matrix (rows "
          "separated by ';')\n"
       << "  ./program -h            Show this help message.\n";
}

vector<vector<int>> parse_matrix_arg(const string& arg) {
  vector<vector<int>> matrix;
  stringstream ss(arg);
  string row;

  while (getline(ss, row, ';')) {
    vector<int> row_values;
    stringstream row_stream(row);
    string value;
    while (getline(row_stream, value, ',')) {
      row_values.push_back(stoi(value));
    }
    matrix.push_back(row_values);
  }

  return matrix;
}

void print_matrix(const vector<vector<int>>& mat) {
  cout << "Grid:\n" << endl;
  for (const auto& row : mat) {
    for (int v : row) cout << v << " ";
    cout << endl;
  }
}

int main(int argc, const char* argv[]) {
  cout << "============================================\n";
  cout << "  Unique Paths Grid Solver (CLI Program)    \n";
  cout << "============================================\n";
  cout << "This program calculates how many unique paths\n";
  cout << "exist in a 2D grid from top-left to bottom-right,\n";
  cout << "moving only right and down, avoiding blocked cells (1).\n\n";

  vector<vector<int>> grid;

  if (argc == 1) {
    cout << "Tip: You can pass a matrix using -m or see help with -h\n";
    cout << "No arguments provided. Using default 3x3 matrix:\n";
    grid = {{0, 0, 0}, {0, 1, 0}, {0, 0, 0}};
  } else if (argc == 2 && string(argv[1]) == "-h") {
    print_help();
    return 0;
  } else if (argc == 3 && string(argv[1]) == "-m") {
    try {
      grid = parse_matrix_arg(argv[2]);
    } catch (...) {
      cerr << "Error parsing matrix. Use format: 0,0,0;0,1,0;0,0,0\n";
      return 1;
    }
  } else {
    cerr << "Invalid arguments. Use -h for help.\n";
    return 1;
  }

  print_matrix(grid);
  int result = unique_paths_in_grid(grid);
  cout << "\n\nResult:\n" << result << endl;

  return 0;
}
