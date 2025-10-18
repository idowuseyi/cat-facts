# 🦀 Cat Facts API Wrapper (Actix + Reqwest)

A simple **Rust web service** built with **Actix-Web** that fetches a random cat fact from the [Cat Facts API](https://catfact.ninja/fact) and returns it in a custom JSON structure along with user metadata and a UTC timestamp.

---

## 🚀 Features

- Fetches a random cat fact from an external API.
- Returns structured JSON response:

  ```json
  {
    "status": "success",
    "user": {
      "email": "idowuseyi22@gmail.com",
      "name": "Oluwaseyi Idowu",
      "stack": "rust"
    },
    "timestamp": "2025-10-17T21:10:43Z",
    "fact": "Cats have five toes on their front paws, but only four on their back paws."
  }

- Built with Actix-Web, Reqwest, Serde, and Chrono.

- Clean async architecture and error handling.

## 🧩 Tech Stack

| Component                               | Purpose                                          |
| --------------------------------------- | ------------------------------------------------ |
| **[Actix-Web](https://actix.rs/)**      | Web framework for handling HTTP requests         |
| **[Reqwest](https://docs.rs/reqwest/)** | HTTP client for fetching data from external APIs |
| **[Serde](https://serde.rs/)**          | Serialization/deserialization of structs         |
| **[Chrono](https://docs.rs/chrono/)**   | Date/time handling for UTC timestamps            |
| **[Tokio](https://tokio.rs/)**          | Asynchronous runtime for Actix and Reqwest       |

## 🛠️ Setup Instructions

1. **Clone the repository**

```bash
git clone https://github.com/<your-username>/cat-facts-api.git
cd cat-facts-api
```

2. **Install Rust (if you haven’t already)**

Install via Rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then confirm installation:

```bash
rustc --version
cargo --version
```

3. **Install dependencies**

The dependencies are managed via Cargo, so they’ll install automatically when you build the project:

```bash
cargo build
```

You can also fetch them manually:

```bash
cargo fetch
```

### 🧾 Environment Variables (Optional)

Currently, no environment variables are required.
However, if you plan to make the user data configurable, you can add a .env file in the project root like this:

```env
USER_EMAIL=idowuseyi22@gmail.com
USER_NAME=Oluwaseyi Idowu
USER_STACK=rust
```

And modify the app to read them using the dotenv crate.

### 🧪 Run Locally

Development mode (auto-reload with cargo-watch)

Install cargo-watch if not already installed:

```bash
cargo install cargo-watch
```

Then run the server:

```bash
cargo watch -x run
```

or run manually

```bash
cargo run
```

The server will start on:

```arduino
http://127.0.0.1:8080/me
```

## 📬 Testing

### ✅ Example Request

```bash
curl http://127.0.0.1:8080/me
```

### ✅ Example Response

```json
{
  "status": "success",
  "user": {
    "email": "idowuseyi22@gmail.com",
    "name": "Oluwaseyi Idowu",
    "stack": "rust"
  },
  "timestamp": "2025-10-17T21:10:43Z",
  "fact": "Cats have whiskers on the backs of their front legs."
}
```

## 🧹 Linting & Formatting

Ensure consistent code style before committing:

```bash
cargo fmt
cargo clippy
```

## 🧱 Project Structure

```bash
cat-facts-api/
├── src/
│   └── main.rs        # Main server file
├── Cargo.toml          # Dependencies and metadata
└── README.md           # Project documentation
```

## Deployment

You can deploy this application to any Rust-compatible hosting service.

You can check out the deployed version with the link below
[Deployed Cat Facts API](https://cat-facts-mkre.shuttle.app/me)

## 📄 License

This project is open-source under the MIT License.

## 👨‍💻 Author

Oluwaseyi Idowu Sunday
Senior Full Stack Developer (Rust + React)
📧 [email](idowuseyi22@gmail.com)
🌐 [LinkedIn](https://www.linkedin.com/in/oluwaseyi-idowu-sunday)
