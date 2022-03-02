# Compiled vs Interpreted
The purposed of this repository is to provide visual examples of two identical programs performing the same task. However one is written using a interpreted language ([Python](https://python.org)) and the other written using a compiled language ([Rust](https://www.rust-lang.org/)). 

Both programs have a GUI using QT, this was done to provide the same graphical interface for both programs. 

The program will calculate prime numbers up to the entered limit, the higher the limit the longer it will take for each program to calculate.

Prime numbers are calculated using an adapted Sieve of Atkin algorithm found on [GeeksforGeeks.org](https://www.geeksforgeeks.org/sieve-of-atkin/)

# Requirements
For running the two programs you will need to install the Rust language, from their [website](https://www.rust-lang.org/tools/install). And Python 3.10 [here](https://www.python.org/downloads/)


# Running the Programs
## Compiled (Rust)
To run the compiled example, open a terminal and navigate to the `compiled` folder within the project, and run the command `cargo run`, this will compile and launch the program window. 

Rust caches already built packages, so running `cargo run` a second time will skip much of the compilation time, running `cargo clean` before launching will ensure everything is rebuilt

## Interpreted (Python)
Before starting the python script, mandatory dependencies for the graphical interface need to be installed first. This can be done by running `pip install -r requirements.txt` from a terminal while navigated to the `interpreted` folder.

To run the program itself open a terminal and navigate to the `compiled` folder within this project. And then run the command `python main.py`, which will launch the program window.