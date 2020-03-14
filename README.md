# rmstuff

A **tiny** tool that cleans up your work directories by deleting build artifacts, unecessary dependencies, and non essential stuff in general.

## Notice

For now it detects only JS projects but I plan to add more types soon. In the case of a NodeJS project it will delete node_modules, public and dist directories, and .cache.

## How to use

1. Compile from source
2. `rmstuff somedir`

To see all the options do `rmstuff --help`.
