# A calculator
## What does it do?
Well, it calculates :D, just like a normal calculator... 

## How to try it :

### The Github Page
Pretty straight-forward and safe, you can head to https://thenil3sh.github.io/calculator/ and give it a try, right in your browser.

### The Releases 
[Release Page]() does exist! Another way is to head there and find a binary, executable for your native system.

### Build from Source
- **Clone this repository to a directory of your choice :**
```shell
git clone https://github.com/thenil3sh/calculator.git
```
_Don't have git installed ? Here's a guide for [installing Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)._

- **Change current directory to just cloned directory** 
```shell
cd calculator
```
- **Start building project with `cargo`**
```shell
cargo build --release
```
_This step requires you to have rustup installed on your system. If you don't have it... this [installation guide](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) gets it done for you._
- **If building ends up successful, there should a binary, `calc` present in `target/release/` , ready to be launched.**
```shell
./target/release/calc
```

## ToDo 
- [ ] Annotate the source code
- [x] Pressing (=) messes up with layout...
- [ ] Make source code even more modular
- [ ] Make it readable (less messy)
- [ ] More easter eggs?
- [ ] (Known Issue) Starting an equation with `(-)` operator doesn't work at all.
- [ ] (Known Issue) Program starts miscalculating if number is too large or too precised
- [ ] (Known Issue) Some (valid) results are way too precised.
- [ ] (Maybe) Add more operations
- [ ] (Maybe) Add other mathematical functions...