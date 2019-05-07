# A library for generating random ids

Inspired by [How Long Does An Id Need To Be](https://eager.io/blog/how-long-does-an-id-need-to-be/), MinimalId is an Id generator that uses nine bytes to generate a mostly unique id with okay locality (to be proved)

## Usage

```rust

use minimal_id::Generator;

let generator = Generator::default();
let id = generator.generate();
println!("Id: {}", id.to_string());

>>> Id: AAECAwQFBgcI
```
