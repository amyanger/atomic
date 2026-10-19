# ⚛️ Atomic - A Simple Programming Language

Welcome to **Atomic**, a **lightweight** and **fun** programming language written in **Rust**! 🚀  
Atomic allows you to execute basic arithmetic operations and print messages to the console. 🖥️✨

---

## 🛠 Features
✅ **Print messages** → `print "Hello, Atomic!"`  
✅ **Perform addition** → `add 5 3` → `8`  
✅ **Perform multiplication** → `multiply 6 7` → `42`  
✅ **Easy to extend** → Add more commands in **Rust**!  

---

## 📦 Installation & Setup

### 1️⃣ **Install Rust** (if you haven't already) 🦀
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2️⃣ **Clone the Atomic Repository** ⚡
```sh
git clone https://github.com/amyanger/atomic-lang.git
cd atomic-lang
```

### 3️⃣ **Build the Project** 🏗️
```sh
cargo build
```

### 4️⃣ **Run Your Atomic Code** 🏃‍♂️
Create a file named **`code.atomic`** with the following content:
```
print "Hello, Atomic!"
add 42 8
multiply 6 7
```

Run it with:
```sh
cargo run code.atomic
```

---

## 📝 Atomic Language Syntax

### 📢 **Printing Text**
```plaintext
print "Hello, Atomic!"
```
🖥️ **Output**:
```
Hello, Atomic!
```

### ➕ **Adding Two Numbers**
```plaintext
add 5 10
```
🖥️ **Output**:
```
5 + 10 = 15
```

### ✖ **Multiplying Two Numbers**
```plaintext
multiply 3 4
```
🖥️ **Output**:
```
3 * 4 = 12
```

---

## 🔧 How It Works
1️⃣ **Lexer**: Converts Atomic code into **tokens** 🧩  
2️⃣ **Parser**: Organizes tokens into an **AST (Abstract Syntax Tree)** 🌲  
3️⃣ **Executor**: Reads the AST and **runs the commands** 🎯  

---

## 💡 Future Improvements
🚀 **Support for subtraction & division**  
🔢 **Variable assignments (`let x = 5`)**  
📝 **User input handling (`input name`)**  
🔁 **Loops & conditionals (`if`, `while`)**  

---

## 🎯 Contributing
Want to **make Atomic even more powerful**? Open a PR! 🛠️🚀  

---

## 📜 License
This project is licensed under the **MIT License**.  

---

 ⚛️

