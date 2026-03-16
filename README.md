# 📂 Rust File Organizer

A simple **Command Line File Organizer written in Rust** that automatically sorts files into folders based on their file type.

This tool scans a directory and moves files into categories like **Images, Music, Documents, and Others**.

This project is part of my **Rust learning journey (Day 7 Project)**.

---

## 🚀 Features

* Automatically organize files by type
* Create folders if they don't exist
* Move files into categorized directories
* Lightweight and fast
* Beginner-friendly Rust filesystem project

---

## 🛠 Built With

* **Rust**
* Rust Standard Library (`std::fs`, `std::path`)

---

## 📂 Project Structure

```text
file_organizer/
│
├── Cargo.toml
├── Cargo.lock
├── files/
│   ├── photo.png
│   ├── song.mp3
│   ├── notes.txt
│   └── report.pdf
│
└── src
    └── main.rs
```

---

## ⚙️ Installation

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/rust-file-organizer.git
```

### 2. Navigate into the folder

```bash
cd rust-file-organizer
```

### 3. Build the project

```bash
cargo build
```

---

## ▶️ Usage

Run the program:

```bash
cargo run
```

The program scans the `files/` directory and organizes files automatically.

---

## 📸 Example

### Before Organizing

```
files/
   photo.png
   song.mp3
   notes.txt
   report.pdf
```

### After Organizing

```
files/
   Images/
       photo.png
   Music/
       song.mp3
   Documents/
       notes.txt
       report.pdf
```

---

## 📸 Output Screenshot

(Add a screenshot of the program output here)

Example placeholder:

```
docs/output.png
```

![Program Output](docs/output.png)

---

## 🧠 Concepts Practiced

This project demonstrates several Rust concepts:

* Directory reading
* File extension detection
* File moving with `fs::rename`
* Path handling with `std::path`
* Error handling
* CLI tool development

---

## 🔮 Future Improvements

Possible improvements:

* Accept folder path from command line
* Support more file types
* Print moved files in terminal
* Add progress output
* Add recursive folder scanning

---

## 📜 License

This project is open source and available under the **MIT License**.

---

## 👨‍💻 Author

**Khurram Rashid**
B.Tech Computer Science Engineering
Amity University
