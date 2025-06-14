### Test file 3

This test defines a custom function in one block and uses it in the main block by importing it.

Define headers:

```c headers
#include <stdio.h>
```

Define a helper function in its own block:

```c helper
void greet(const char* name) {
    printf("Hello, %s!\n", name);
}
```

Define main block `run`:
```c use=[headers,helper] run
    greet("Tangle User");
```
