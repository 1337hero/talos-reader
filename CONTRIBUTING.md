# Contributing to Talos

Thanks for your interest in contributing! 🎉  
Talos is an open-source project, and contributions of all kinds are welcome — from bug fixes and new features to docs, examples, and discussions.

---

## How to Contribute

1. **Fork the repository**  
   Create your own fork of the repo on GitHub.

2. **Create a feature branch**  
   Use a descriptive name for your branch:  
   ```bash
   git checkout -b feature/add-go-extractor
   ```

3. **Make your changes**  
   Keep commits focused and meaningful. Follow existing code style where possible.

4. **Add tests (if applicable)**  
   If your change affects functionality, include or update tests to cover it.

5. **Run the build**  
   Verify everything compiles and passes before submitting:  
   ```bash
   cargo build
   cargo test
   ```

6. **Submit a pull request**  
   Push your branch and open a PR against the `main` branch.  
   Provide a clear description of what your change does and why.

---

## Development Setup

- Talos is written in **Rust**. Install it via [rustup](https://rustup.rs/) if you don’t already have it.
- Build with:
  ```bash
  cargo build
  ```
- Run tests with:
  ```bash
  cargo test
  ```

---

## Guidelines

- Keep PRs focused on a single feature/fix when possible.
- Update documentation if your changes affect usage.
- Be respectful and constructive in code reviews and discussions.

---

Built with ❤️ by [1337Hero](https://github.com/1337hero)
