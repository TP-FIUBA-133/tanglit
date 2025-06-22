### Test file

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

```c use=[headers,helper] main_block
    greet("Tangle User");
```

---

You can also import blocks using macros inside the code block:

Define a variable:

```c config
const char* language = "C";
```

Import and use the variable using a macro:

```c variable_user use=[headers]
    @[config]
    printf("This program is written in %s.\n", language);
```
