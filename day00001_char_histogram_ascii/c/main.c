#include <stdio.h>
#include <string.h>

#define MAX_LEN 256

void char_histogram(const char str[], int freq[]) {
  size_t n = strlen(str);
  size_t index;
  for (size_t i = 0; i < n; i++) {
    // it's uppercase
    if ('A' <= str[i] && str[i] <= 'Z') {
      index = str[i] - 'A';
      freq[index] += 1;
    } else if (96 < str[i] && str[i] <= 122) {
      index = (size_t)str[i] - 'a';
      freq[index] += 1;
    }
  }
}

void print_char_histogram(const int freq[]) {
  int counter;
  for (size_t i = 0; i < 26; i++) {
    counter = freq[i];
    if (counter <= 0) continue;

    printf("%c: ", (char)(i + 'a'));

    while (0 < counter--) printf("*");

    printf("\n");
  }
}

int main(const int argc, const char *argv[]) {
  char input[MAX_LEN];
  printf("Welcome to char_histogram_ascii!\n");

  if (argc == 1) {
    printf("Enter input text: \n\n");
    if (fgets(input, MAX_LEN, stdin) == NULL) {
      fprintf(stderr, "Error: input error\n");
      return 1;
    }
    printf("\n\n");

    // delete \n from str
    size_t len = strlen(input);
    if (len > 0 && input[len - 1] == '\n') {
      input[len - 1] = '\0';
    }
  } else if (argc == 2) {
    printf("Analyzing input from command-line argument...\n\n");

    size_t len = strlen(argv[1]);

    // check size string
    if (len >= MAX_LEN) {
      fprintf(stderr, "Error: the input value exceeded %d characters\n",
              MAX_LEN - 1);
      return 1;
    }

    strncpy(input, argv[1], MAX_LEN - 1);
    input[MAX_LEN - 1] = '\0';  // to be sucure strings end in '\0'
  } else {
    fprintf(stderr, "Error: the program only accepts one optional argument\n");
    return 1;
  }

  int freq[26] = {0};
  char_histogram(input, freq);
  print_char_histogram(freq);
  return 0;
}
