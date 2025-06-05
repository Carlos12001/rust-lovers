#include <stdio.h>
#include <string.h>

#define MAX_LEN 256

int count_vowels(const char s[]) {
  int num_vowels = 0;
  const size_t n = strlen(s);
  const char vowels[] = "AEIOUaeiou";
  const size_t n_vowels = strlen(vowels);
  for (size_t i = 0; i < n; i++) {
    for (size_t j = 0; j < n_vowels; j++) {
      if (s[i] == vowels[j]) {
        num_vowels++;
        continue;
      }
    }
  }
  return num_vowels;
}

int main(int argc, char *argv[]) {
  char s[MAX_LEN];
  if (argc == 1) {
    printf("Enter input text: \n\n");

    if (fgets(s, MAX_LEN, stdin) == NULL) {
      fprintf(stderr, "Error: input error\n");
      return 1;
    }
    printf("\n");
    size_t n = strlen(s);
    if (n > 0 && s[n - 1] == '\n') {
      s[n - 1] = '\0';
    }
  } else {
    const size_t n = snprintf(s, MAX_LEN, "%s", argv[1]);
    if (n >= MAX_LEN) {
      fprintf(stderr, "Warning: input truncated to %d characters.\n",
              MAX_LEN - 1);
    }
  }
  int vowel_num = count_vowels(s);
  fprintf(stdout, "The number of vowels is\n%d\n", vowel_num);
  return 0;
}
