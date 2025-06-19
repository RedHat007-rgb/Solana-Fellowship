# 🧮 Basic Rust Calculator with Solana Token Conversion

## 🚀 Overview

This project is a simple command-line calculator written in Rust that supports basic arithmetic operations: addition, subtraction, multiplication, and division. After each calculation, it converts the result into a hypothetical Solana (SOL) token value based on a fixed conversion rate:

> 💱 **Conversion Rate:** `1 SOL = 100 units` of the calculator’s result (e.g., 8.0 → 0.08 SOL)

This assignment is designed to reinforce fundamental Rust concepts, such as:

- Data types and operations
- Functions and error handling
- Conditionals and loops
- Mutability
- User input and string parsing

---

## 🛠 Features

- ✅ Addition, Subtraction, Multiplication, Division
- ✅ Error handling for division by zero
- ✅ Dynamic user input via command line
- ✅ Continuous operation using a loop
- ✅ SOL token value conversion from the result

---

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install)

To check if Rust is installed:

```bash
rustc --version
If not installed, you can install it via rustup:

bash
Copy
Edit
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

▶️ How to Run
Clone the repository:

bash
Copy
Edit
git clone https://github.com/your-username/rust-solana-calculator.git
cd rust-solana-calculator
Build and run the project:

bash

cargo run


Example interaction:

Enter operation (add/sub/mul/div): add
Enter first number: 5.0
Enter second number: 3.0
Result: 8.0
Equivalent in SOL: 0.08 SOL
Do another calculation? (yes/no): yes


📂 Project Structure

bash
Copy
Edit
├── src
│   └── main.rs      # Main logic for calculator
├── Cargo.toml       # Rust project config
└── README.md        # Project documentation
📚 Concepts Covered
Primitive Types: i32, f64

Functions: Custom functions for arithmetic operations

Match/If-Else: Operation selector

Looping: Repeated calculations until exit

Mutability: Dynamic user input handling

Error Checking: Division-by-zero handling

Input Parsing: From String to f64

🤝 Contributing
Pull requests and forks are welcome. For major changes, please open an issue first to discuss what you would like to change.

