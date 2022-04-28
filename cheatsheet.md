1. speed up linking => faster linking. lld for llvm
2. Cargo watch -> Speed up complilation time.
cargo-watch monitors your source code to trigger commands every time a file changes.
```
cargo watch -x check
or 
cargo watch -x check -x test -x run
```
3. put CI in our applications.
CI steps: 
> 1. Test 
> 2. Code coverage 
> 3. Linting
> 4. Formating
> 5. Security checks