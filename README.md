# Simon Says Generator

## Introduction
This is a simple Rust program that generates a phrase using the "Simon Says" game concept. The program takes command-line arguments and randomly decides whether to prepend "Simon Says" to the input phrase.

## Prerequisites
- Rust programming language
- Cargo package manager

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/simon_says.git
   cd simon_says
   ```

2. Build the project using Cargo:
   ```bash
   cargo build
   ```

3. Run the executable with command-line arguments:
   ```bash
   ./target/debug/simon_says Hello World
   ```
   - The program will randomly decide whether to prepend "Simon Says" to the input phrase.

## Usage
- The program is designed to take any number of command-line arguments and generate a phrase based on the "Simon Says" game.
- If no arguments are provided, the program will print an empty line.

## Example
```bash
./target/debug/simon_says Learning Rust is fun
```
Output:
```
Simon Says Learning Rust is fun
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments
- This program uses the `rand` crate for random number generation.

Feel free to modify and use the code as needed! If you encounter any issues or have suggestions, please open an issue on the GitHub repository. Contributions are welcome!
