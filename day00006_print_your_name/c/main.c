#include <stdio.h>
#include <string.h>

#define MAX_LEN 256
int main(int argc, char *argv[]) {
  char name[MAX_LEN];
  if (argc == 1) {
    printf("Please enter your name:\n");
    if (fgets(name, MAX_LEN, stdin) == NULL) {
      fprintf(stderr, "\nError: to read line.");
      return 1;
    }
    printf("\n");
    size_t len = strlen(name);
    if (len > 0 && name[len - 1] == '\n') {
      name[len - 1] = '\0';
    }
  } else {
    const size_t n = snprintf(name, MAX_LEN, "%s", argv[1]);
    if (n >= MAX_LEN) {
      fprintf(stderr, "Warning: input truncated to %d characters.\n",
              MAX_LEN - 1);
    }
  }
  printf("Your name is: %s\n", name);
  return 0;
}