# 📚 Student Grade Tracker CLI (Rust)

A simple command-line tool built in Rust to track a student's subjects and marks, calculate total and average, and assign a grade — made for Rust beginners! 🦀

---

## 🚀 Features

- Take input: student name, subjects, and marks
- Display a summary report:
  - Subject-wise marks
  - Total and average
  - Grade (A/B/C/F)
- Runs entirely in the terminal
- Beginner-friendly, built using only basic Rust concepts

---

## 💡 Concepts Practiced

This project only uses topics from Chapters 1 to 4 of _The Rust Programming Language_:

- Variables & Mutability
- Data Types (`String`, `i32`, `f64`, `Vec`)
- Functions
- Control Flow (`if-else`, `loop`, `for`)
- Ownership and Borrowing

---

## 📥 Input Format

The app will ask for:

1. Student name
2. Number of subjects
3. For each subject:
   - Subject name
   - Marks

---

## 📤 Output Example

Enter student name: Alice
How many subjects? 3

Enter subject 1 name: Math
Enter marks for Math: 90

Enter subject 2 name: Science
Enter marks for Science: 85

Enter subject 3 name: History
Enter marks for History: 75

----------- Report Card -----------
Name: Alice
Subjects and Marks:
Math - 90
Science - 85
History - 75

Total Marks: 250
Average: 83.33
Grade: B
