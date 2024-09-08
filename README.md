# meeworks

# To Build
```
cargo build
```

# To Run 
```
cargo run
```

# Add Dependencies: 

You can add dependencies to your project by editing the Cargo.toml file. For example, to add the serde library for serialization, you would add:
```
[dependencies]
serde = "1.0"
```

# Update Dependencies: 
To update your project’s dependencies to the latest versions, run:
```
cargo update
```

# Test Your Code: 
Cargo makes it easy to run tests with:
```
cargo test
```

# Build for Release: 

When you’re ready to build an optimized version of your project, use:
```
cargo build --release
```
This creates an optimized executable in the target/release directory.
