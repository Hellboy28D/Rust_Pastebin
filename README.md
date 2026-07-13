# 🦀 Rust PasteBin

<img width="816" height="937" alt="Screenshot 2026-07-13 at 1 49 41 PM" src="https://github.com/user-attachments/assets/c00f6785-7410-4ef8-a6c6-95266b6d34a7" />



A fast, lightweight, and minimal **Pastebin clone** built entirely with **Rust**, **Actix Web**, and **SQLite**. This project allows users to submit text or code snippets and instantly receive a unique URL to access their paste.

Designed with simplicity, performance, and security in mind, Rust PasteBin demonstrates how modern Rust web development can be used to build efficient backend services with minimal dependencies.

---

## ✨ Features

- 🚀 Built using the Actix Web framework
- 🗄️ SQLite database for persistent storage
- 🔑 Random unique token generation for every paste
- 📄 Individual URL for every submitted paste
- 🎨 Responsive dark-themed interface
- ⚡ Fast and lightweight
- 🔒 HTML escaping to prevent XSS attacks
- 📱 Mobile-friendly layout
- 🦀 Entire backend written in Rust
- 💾 Automatic database creation on startup

---

# 📸 Preview

## Home Page

Users can paste any text or code into the editor.

```
+---------------------------------------+
|                                       |
|        Rust PasteBin                  |
|                                       |
|  +-------------------------------+    |
|  |                               |    |
|  |                               |    |
|  |         Paste Here            |    |
|  |                               |    |
|  +-------------------------------+    |
|                                       |
|        [ Submit ]                     |
+---------------------------------------+
```

---

## Generated Paste

After submission:

```
http://localhost:8080/paste/Ab3kLm91PwXqT2Yz
```

Displays

```
fn main() {
    println!("Hello, Rust!");
}
```

---

# 🛠️ Tech Stack

| Technology | Purpose |
|------------|---------|
| Rust | Programming Language |
| Actix Web | Web Framework |
| SQLite | Database |
| Rusqlite | SQLite bindings |
| HTML | Frontend |
| CSS | Styling |
| Serde | Form Deserialization |
| Rand | Random token generation |
| html-escape | XSS Protection |

---

# 📂 Project Structure

```
Rust_Pastebin/
│
├── Cargo.toml
├── README.md
├── pastes.db
│
└── src/
    ├── main.rs
    ├── index.html
    └── style.css
```

---

# ⚙️ Installation

Clone the repository

```bash
git clone https://github.com/Hellboy28D/Rust_Pastebin.git
```

Go into the project

```bash
cd Rust_Pastebin
```

Build the project

```bash
cargo build
```

Run the server

```bash
cargo run
```

The application will be available at

```
http://127.0.0.1:8080
```

---

# 🚀 Usage

### Step 1

Open

```
http://127.0.0.1:8080
```

---

### Step 2

Paste your text or code.

Example

```rust
fn main() {
    println!("Hello World");
}
```

---

### Step 3

Click **Submit**

---

### Step 4

You will be redirected to

```
/paste/<random-token>
```

Example

```
http://127.0.0.1:8080/paste/N4hT8LpQxV2Ke7Zm
```

---

# 🗄️ Database

The application automatically creates an SQLite database named

```
pastes.db
```

with the following schema

```sql
CREATE TABLE pastes (
    token TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

---

# 🔒 Security

Current security features include

- HTML escaping before rendering
- Unique random paste tokens
- SQL injection protection using prepared statements
- Server-side form handling
- Safe database access with Mutex synchronization

---

# ⚡ Performance

Rust and Actix Web provide exceptional performance.

- Memory safe
- Zero-cost abstractions
- Fast request handling
- Lightweight SQLite backend
- Minimal runtime overhead

---

# 📦 Dependencies

```toml
actix-web
actix-files
rusqlite
serde
rand
html-escape
```

---

# 🔮 Future Improvements

- User authentication
- Syntax highlighting
- Copy-to-clipboard button
- Password protected pastes
- Paste expiration
- Markdown support
- File uploads
- Dark/Light mode toggle
- Search functionality
- Paste editing
- Delete pastes
- REST API
- Docker support
- PostgreSQL support
- Redis caching
- Rate limiting
- HTTPS deployment
- Custom URLs
- QR code generation
- Download as file
- Admin dashboard
- User profiles

---

# 🧠 What I Learned

This project helped strengthen my understanding of

- Rust ownership and borrowing
- Async programming
- Actix Web routing
- SQLite integration
- Form handling
- HTTP request/response lifecycle
- Random token generation
- Backend architecture
- Web security fundamentals
- Building full-stack applications with Rust

---

# 🤝 Contributing

Contributions are welcome.

1. Fork the repository

2. Create a feature branch

```bash
git checkout -b feature/new-feature
```

3. Commit your changes

```bash
git commit -m "Add awesome feature"
```

4. Push to GitHub

```bash
git push origin feature/new-feature
```

5. Open a Pull Request

---

# 📄 License

This project is licensed under the MIT License.

---

# 👨‍💻 Author

**Hellboy28D**

GitHub: https://github.com/Hellboy28D

If you found this project helpful, consider giving it a ⭐ on GitHub!

---

## ⭐ Star the Repository

If you enjoyed this project or found it useful, please consider leaving a **star** on GitHub. It helps others discover the project and motivates further development.

Happy Coding! 🦀
