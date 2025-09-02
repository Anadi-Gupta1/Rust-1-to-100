# 🦀 Rust-1-to-100: Complete Learning Journey

<div align="center">

![Rust Logo](https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg)

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![GitHub](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Anadi-Gupta1/Rust-1-to-100)
[![License](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](LICENSE)

**🚀 A comprehensive journey through the Rust programming language, from zero to hero! 🚀**

*Master Rust step by step with hands-on examples, practical exercises, and real-world applications.*

</div>

---

## 📋 Table of Contents

- [🏗️ Project Structure](#️-project-structure)
- [🚀 Quick Start](#-quick-start)
- [📚 Learning Path](#-learning-path)
- [🛠️ How to Run](#️-how-to-run)
- [📂 File Organization](#-file-organization)
- [🎯 Learning Goals](#-learning-goals)
- [🤝 Contributing](#-contributing)
- [📜 License](#-license)

---

## 🏗️ Project Structure

```
📦 helloRust/
├── 📁 src/
│   ├── 📁 examples/          # Basic Examples & Hello World
│   │   ├── 🦀 main.rs        # Hello World - Your first Rust program
│   │   ├── 🦀 main2.rs       # Variables & String formatting
│   │   └── 🦀 main3.rs       # Mutable variables demonstration
│   │
│   ├── 📁 concepts/          # Core Rust Concepts
│   │   ├── 🦀 datatypespractice.rs    # Hands-on with data types
│   │   ├── 🦀 rustconstant.rs         # Constants and immutability
│   │   ├── 🦀 rust_datatypes.rs       # Data types deep dive
│   │   └── 🦀 rust_datastructure.rs   # Data structure fundamentals
│   │
│   ├── 📁 data_structures/   # Advanced Data Structures
│   │   ├── 🦀 rust_array.rs       # Arrays and fixed collections
│   │   ├── 🦀 rust_vector.rs      # Dynamic vectors (Vec<T>)
│   │   ├── 🦀 rust_hashmap.rs     # Key-value storage (HashMap)
│   │   ├── 🦀 rust_tuples.rs      # Tuples and compound types
│   │   ├── 🦀 rust_structs.rs     # Custom data structures
│   │   └── 🦀 rust_enums.rs       # Enumerations and pattern matching
│   │
│   ├── 📁 advanced_concepts/ # Advanced Rust Features
│   │   ├── 🦀 Enums & Pattern Matching.rs    # Advanced enums & pattern matching
│   │   ├── 🦀 Mutability, Memory Management, Ownership.rs  # Core Rust ownership system
│   │   ├── 🦀 Referencing & borrowing.rs     # References, borrowing & lifetimes
│   │   ├── 🦀 struct.rs                      # Advanced struct implementations
│   │   ├── 🦀 pattern matching.rs           # Pattern matching techniques
│   │   └── 🦀 errorhandling.rs              # Result, Option & error handling
│   │
│   └── 📁 bin/               # Executable Programs
│       ├── 🦀 example.rs         # Template for new programs
│       ├── 🦀 rustoperator.rs    # Comprehensive operators guide
│       └── 🦀 test.rs            # Testing playground
│
├── 📄 Cargo.toml            # Project configuration
├── 📄 Cargo.lock            # Dependency lockfile
└── 📖 README.md             # You are here!
```

---

## 🚀 Quick Start

### Prerequisites
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Set up the GNU toolchain (Windows users)
rustup default stable-x86_64-pc-windows-gnu
```

### Clone and Run
```bash
# Clone the repository
git clone https://github.com/Anadi-Gupta1/Rust-1-to-100.git
cd Rust-1-to-100/helloRust

# Run your first Rust program
cargo run --bin main

# Try the operators demonstration
cargo run --bin rustoperator
```

---

## 📚 Learning Path

### 🌱 **Beginner Level** (Start Here!)
1. **Hello World** → `cargo run` (examples/main.rs)
2. **Variables & Strings** → `cargo run --bin main2`  
3. **Mutable Variables** → `cargo run --bin main3`
4. **Data Types Practice** → `cargo run --bin datatypespractice`

### 🌿 **Intermediate Level**
5. **Constants** → `cargo run --bin rustconstant`
6. **Operators** → `cargo run --bin rustoperator`
7. **Arrays** → `cargo run --bin rust_array`
8. **Vectors** → `cargo run --bin rust_vector`
9. **Tuples** → `cargo run --bin rust_tuples`
10. **HashMaps** → `cargo run --bin rust_hashmap`

### 🌳 **Advanced Level**
11. **Structs** → `cargo run --bin rust_structs`
12. **Enums & Pattern Matching** → Learn from `src/advanced_concepts/Enums & Pattern Matching.rs`
13. **Memory Management & Ownership** → Study `src/advanced_concepts/Mutability, Memory Management, Ownership.rs`
14. **References & Borrowing** → Explore `src/advanced_concepts/Referencing & borrowing.rs`
15. **Advanced Structs** → Check `src/advanced_concepts/struct.rs`
16. **Pattern Matching Techniques** → Review `src/advanced_concepts/pattern matching.rs`
17. **Error Handling** → Master `src/advanced_concepts/errorhandling.rs`

---

## 🛠️ How to Run

### 🎯 **Easy Method** (For any file in `src/bin/`)
```bash
# Just put your .rs file in src/bin/ and run:
cargo run --bin filename
```
*No need to modify Cargo.toml!*

### 🔧 **Development Workflow**
```bash
# 1. Write your code in src/bin/newfile.rs
# 2. Run it immediately
cargo run --bin newfile

# 3. Push to GitHub (one-liner)
git add . ; git commit -m "Update: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" ; git push origin main
```

### 💡 **Available Commands**
| Command | Description | Example |
|---------|-------------|---------|
| `cargo run` | Run main.rs | `cargo run` |
| `cargo run --bin <name>` | Run specific binary | `cargo run --bin rustoperator` |
| `cargo build` | Build project | `cargo build` |
| `cargo check` | Check for errors | `cargo check` |

### 📖 **Advanced Topics Quick Reference**
| Topic | File Location | Key Concepts |
|-------|---------------|--------------|
| **Ownership** | `src/advanced_concepts/Mutability, Memory Management, Ownership.rs` | Move semantics, Drop trait |
| **Borrowing** | `src/advanced_concepts/Referencing & borrowing.rs` | References, mutable/immutable borrows |
| **Pattern Matching** | `src/advanced_concepts/Enums & Pattern Matching.rs` | Match expressions, destructuring |
| **Error Handling** | `src/advanced_concepts/errorhandling.rs` | Result<T,E>, Option<T>, ? operator |
| **Advanced Structs** | `src/advanced_concepts/struct.rs` | Methods, associated functions |

---

## 📂 File Organization

| Directory | Purpose | File Count |
|-----------|---------|------------|
| 📁 **examples/** | Basic Rust programs | 5 files |
| 📁 **concepts/** | Core language concepts | 4 files |
| 📁 **data_structures/** | Advanced data structures | 6 files |
| 📁 **advanced_concepts/** | Advanced Rust features | 6 files |
| 📁 **bin/** | Executable binaries | 3+ files |

---

## 🎯 Learning Goals

### ✅ **Completed Topics**
- [x] Hello World & Basic Syntax
- [x] Variables & Mutability  
- [x] Data Types (integers, floats, strings, booleans)
- [x] Constants & Immutability
- [x] Operators (arithmetic, comparison, logical, assignment)
- [x] Arrays & Vectors
- [x] Tuples & Compound Types
- [x] HashMaps & Key-Value Storage
- [x] Structs & Custom Data Types
- [x] Enums & Pattern Matching (Advanced)
- [x] Memory Management & Ownership System
- [x] References & Borrowing
- [x] Error Handling (Result, Option)
- [x] Advanced Pattern Matching Techniques

### 🔄 **In Progress**
- [ ] Functions & Control Flow (Advanced)
- [ ] Lifetimes (Deep Dive)
- [ ] Traits & Generics

### 📋 **Coming Soon**
- [ ] Modules & Crates
- [ ] Testing
- [ ] Concurrency
- [ ] Async Programming
- [ ] Web Development with Rust
- [ ] Performance Optimization

---

## 🤝 Contributing

Want to contribute to this learning journey? Here's how:

1. **Fork** the repository
2. **Create** a new branch: `git checkout -b feature/new-concept`
3. **Add** your Rust examples in the appropriate directory
4. **Commit** your changes: `git commit -m "Add: New Rust concept"`
5. **Push** to the branch: `git push origin feature/new-concept`
6. **Open** a Pull Request

### 💡 **Contribution Ideas**
- Add more practical examples
- Improve documentation
- Add exercises with solutions
- Create advanced topics
- Fix bugs or typos

---

## 📊 **Repository Stats**

<div align="center">

![GitHub stars](https://img.shields.io/github/stars/Anadi-Gupta1/Rust-1-to-100?style=social)
![GitHub forks](https://img.shields.io/github/forks/Anadi-Gupta1/Rust-1-to-100?style=social)
![GitHub watchers](https://img.shields.io/github/watchers/Anadi-Gupta1/Rust-1-to-100?style=social)

**📈 Learning Progress:** 20+ Rust concepts covered and growing!

</div>

---

## 📜 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 📧 Contact & Support

<div align="center">

**🔗 Connect with me:**

[![GitHub](https://img.shields.io/badge/GitHub-Anadi--Gupta1-black?style=flat-square&logo=github)](https://github.com/Anadi-Gupta1)
[![Email](https://img.shields.io/badge/Email-Contact-red?style=flat-square&logo=gmail)](mailto:anadi@example.com)

**⭐ If this repository helped you learn Rust, please give it a star! ⭐**

</div>

---

<div align="center">

**🦀 Happy Rust Learning! 🦀**

*"Rust is a language that empowers everyone to build reliable and efficient software."*

---

Made with ❤️ by [Anadi Gupta](https://github.com/Anadi-Gupta1)

</div>
