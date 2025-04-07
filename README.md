# **Touch for Windows**

A Rust-based command-line utility that replicates the functionality of the Linux `touch` command for Windows. This tool allows users to create new files.
---

## **Features**
- Create new empty files if they do not already exist.
- Update the access and modification timestamps of existing files.
- Prevent overwriting or modifying existing files.
- Simple and fast, with robust error handling.
- Cross-platform compatibility (designed for Windows but can work on other platforms).
---

## **Installation**
- Download from github release.
- Add the executable to your system PATH.

### **Build** 
1. **Clone the Repository**:
   ```bash
   git clone https://github.com/jashmevada/touch-windows.git
   cd touch-windows
   ```

2. **Build the Project** (requires Rust installed):
   ```bash
   cargo build --release
   ```

3. **Run the Executable**:
   The compiled executable will be located in the `target/release` directory:
   ```bash
   ./target/release/touch
   ```

4. (Optional) Add the executable to your system PATH for easier usage.

---

## **Usage**

### **Basic Syntax**
```bash
touch 
```

### **Examples**

1. **Create a New File**:
   ```bash
   touch myfile.txt
   ```
   - If `myfile.txt` does not exist, it will be created.
   - If `myfile.txt` exists, its timestamps will be updated.

2. **Prevent Overwriting Existing Files**:
   ```bash
   touch myfile.txt
   ```
   - If `myfile.txt` already exists, no changes will be made, and a message will be displayed.

3. **Error Handling**:
   - If the program cannot create or open a file due to permissions or other issues, it will display an appropriate error message.

---

## **How It Works**

This tool uses Rust's standard library to handle file operations safely and efficiently:
- **File Creation**: Uses `OpenOptions::new()` with `.create_new(true)` to ensure files are only created if they don't already exist.
- **File Existence Check**: Uses `metadata()` to determine if a file already exists before attempting any operations.
- **Error Handling**: Provides clear error messages when operations fail (e.g., due to insufficient permissions).

---

## **Why Use This Tool?**

While the Linux `touch` command is widely available, there isn't a native equivalent for Windows. This tool fills that gap by providing similar functionality in a lightweight and portable Rust-based utility.

### Advantages:
- Lightweight and fast.
- Cross-platform compatibility.
- Simple and intuitive usage.
- Written in Rust, ensuring safety and performance.

---

## **Contributing**

Contributions are welcome! If you find a bug or have suggestions for improvements, please open an issue or submit a pull request.

### Steps to Contribute:
1. Fork this repository.
2. Create a new branch for your feature or bug fix.
3. Commit your changes and push them to your fork.
4. Open a pull request describing your changes.

---

## **License**

This project is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this software as per the terms of the license.

---

## **Acknowledgments**

This tool was inspired by the Linux `touch` command and aims to bring similar functionality to Windows users in an efficient manner using Rust.

