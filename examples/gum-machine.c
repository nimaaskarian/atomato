#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <editline/readline.h>

static char * states[] = {
 "s0", "s0", "s0", "s0", "s0", "s1", "s1", "s1", "s1", "s1", "s2", "s2", "s2", "s2", "s2", "s3", "s3", "s3", "s3", "s3", "s4", "s4", "s4", "s4", "s4", NULL
};

static char * inputs[] = {
 "5", "10", "25", "w", "b", "5", "10", "25", "w", "b", "5", "10", "25", "w", "b", "5", "10", "25", "w", "b", "w", "b", "5", "10", "25", NULL
};

static char * ostates[] = {
 "s1", "s2", "s4", "s0", "s0", "s2", "s3", "s4", "s1", "s1", "s3", "s4", "s4", "s2", "s2", "s4", "s4", "s4", "s3", "s3", "s0", "s0", "s4", "s4", "s4", NULL
};

static char * outputs[] = {
 "n", "n", "5", "n", "n", "n", "n", "10", "n", "n", "n", "n", "15", "n", "n", "n", "5", "10", "n", "n", "p", "s", "5", "10", "25", NULL
};

void process_input(const char *input, size_t len) {
  char * state = "s0";
  size_t strindex = 0;
  while(strindex < len) {
    for (int i = 0; inputs[i] != NULL; i++) {
      size_t step = strlen(inputs[i]);
      if (strncmp(inputs[i], input, step) == 0 &&
        strcmp(state, states[i]) == 0) {
        fprintf(stderr, "%s -> %s\n", state, ostates[i]);
        state = ostates[i];
        printf("%s", outputs[i]);
        strindex+=step;
        input+=step;
        break;
      }
    }
  }
}

int main(int argc, char *argv[]) {
  if (isatty(STDIN_FILENO)) {
    while (1) {
        char * line = readline("> ");
        process_input(line, strlen(line));
        puts("");
    }
  } else {
    char line[256];          
    while (fgets(line, sizeof(line), stdin) != NULL) {
      size_t size = strcspn(line, "\n");
      line[size] = 0;
      process_input(line, size);
      puts("");
    }
  }
  return 0;
}
