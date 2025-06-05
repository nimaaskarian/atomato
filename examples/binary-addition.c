#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <editline/readline.h>

static char * states[] = {
 "s0", "s0", "s0", "s0", "s1", "s1", "s1", "s1", NULL
};

static char * inputs[] = {
 "00", "10", "01", "11", "00", "01", "10", "11", NULL
};

static char * ostates[] = {
 "s0", "s0", "s0", "s1", "s0", "s1", "s1", "s1", NULL
};

static char * outputs[] = {
 "0", "1", "1", "0", "1", "0", "0", "1", NULL
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
