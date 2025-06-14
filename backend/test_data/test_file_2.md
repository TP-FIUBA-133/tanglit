### Test file 2

With this file, we want to test the tangle functionality using metadata imports.

Defining a block with name `stdio`:

```c stdio
#include <stdio.h>
```

Importing the block to print "Hello world!":

```c print use=[stdio]
    printf("Hello, World!\n");
```


