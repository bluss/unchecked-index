
# unchecked-indexing

Unchecked indexing through the regular index syntax.

Using a wrapper type that requires an `unsafe` block to create.

## Example

```rust

use unchecked_index::unchecked_index;

/// unsafe because: trusts the permutation to be correct
unsafe fn apply_permutation<T>(perm: &mut [usize], v: &mut [T]) {
    debug_assert_eq!(perm.len(), v.len());
    
    // use unchecked (in reality, debug-checked) indexing throughout
    let mut perm = unchecked_index(perm);
    
    for i in 0..perm.len() {
        let mut current = i;
        while i != perm[current] {
            let next = perm[current];
            // move element from next to current
            v.swap(next, current);
            perm[current] = current;
            current = next;
        }
        perm[current] = current;
    }
}
```

## How to contribute:

- Fix a bug or implement a new thing
- Include tests for your new feature
- Make a pull request


## Recent Changes

- 0.1.0

  - Initial release
