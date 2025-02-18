In Rust, you can save and load data to/from a file using the `std::fs` module. Below are examples of how to do this:

---

### **1. Writing to a File**
To save data to a file, you can use `std::fs::write()` or `std::io::Write`.

```rust
use std::fs::File;
use std::io::Write;

fn save_to_file(filename: &str, data: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?; // Create or overwrite the file
    file.write_all(data.as_bytes())?; // Write data as bytes
    Ok(())
}

fn main() {
    match save_to_file("data.txt", "Hello, Rust!") {
        Ok(_) => println!("Data saved successfully."),
        Err(e) => eprintln!("Error saving data: {}", e),
    }
}
```

---

### **2. Reading from a File**
To load data from a file, you can use `std::fs::read_to_string()`.

```rust
use std::fs::read_to_string;
use std::io;

fn load_from_file(filename: &str) -> io::Result<String> {
    let content = read_to_string(filename)?; // Read the file content
    Ok(content)
}

fn main() {
    match load_from_file("data.txt") {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
```

---

### **3. Saving and Loading Structured Data (Using Serde for JSON)**
If you want to save and load structured data (e.g., structs), you can use **Serde** with JSON.

#### **Add Dependencies to `Cargo.toml`**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### **Rust Code for Saving/Loading JSON**
```rust
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn save_to_file(filename: &str, person: &Person) -> Result<()> {
    let json = serde_json::to_string(person)?; // Serialize struct to JSON
    fs::write(filename, json)?; // Save JSON to file
    Ok(())
}

fn load_from_file(filename: &str) -> Result<Person> {
    let json = fs::read_to_string(filename)?; // Read JSON from file
    let person: Person = serde_json::from_str(&json)?; // Deserialize JSON to struct
    Ok(person)
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let filename = "person.json";

    // Save to file
    if let Err(e) = save_to_file(filename, &person) {
        eprintln!("Error saving: {}", e);
    }

    // Load from file
    match load_from_file(filename) {
        Ok(person) => println!("Loaded Person: {:?}", person),
        Err(e) => eprintln!("Error loading: {}", e),
    }
}
```

---

### **Summary**
1. **Basic text saving/loading:**
   - Use `std::fs::write()` to save a string.
   - Use `std::fs::read_to_string()` to load a string.
2. **Structured data (JSON serialization/deserialization):**
   - Use **Serde** to serialize Rust structs into JSON.
   - Use `serde_json::to_string()` to convert a struct to JSON.
   - Use `serde_json::from_str()` to convert JSON back into a struct.
