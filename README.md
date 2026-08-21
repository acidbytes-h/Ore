# Oxide
<img src="oxidelogo.svg" alt="Alt text" width="400">

*The logo for Oxide, which was inspired by Rust's Ferris mascot.*

Oxide is a programming language that is 90% based on **Rust**. Which is actually mainly because it was built on Rust! The remaining 10% is inspired by *JS*,
```rust
sleep(1000) // This equals to 1 second, like in JS!
```
*C*,
```c
\n // These are C-styled comments... that work in Rust as well, but whatever.
```
and *Swift!*
```rust
let number = 21
```
However, that by no way means it doesn't bring it's own flavour!

# Hello World!
A typical Hello World program looks like this :
```cpp
println?("Hello World!")
```
Or :
```cpp
print?("Hello World\n")
```
It's half serious, half joke. Because :
* *Rust : println!*
* *Oxide : println?*

Yep. It got changed by one letter.

Well, it at least does what it has to.
# Comments
Comments are the same as you'd do in almost any programming language :
```c
// This is a comment! Nothing special about it.
```
# Variables
```rust
let meaning_of_life = 42
// And how to print those variables?
println?(meaning_of_life)
```
# Timer
As with any sane language, Oxide has a timer!
```c
sleep?(1000) // This equals to 1 second!
```
More on this further down.
# User Input
This is the most important part of Oxide (as of now) :
```rust
println?("What is your dev name?")
let name = input?()
println?("Hello, ")
println?(name)
```
In my opinion, it is an absolute necessity to make your programming language have input.

Also, yes. It uses ? again.

Honestly, this is the biggest thing that puts it apart from Rust as of this version.
# Requirements
To actually get to use Oxide, you need to download rustc (don't ask me, what am I, a README.md?)

Then, you need to download the source file that's right in this repo.

Then compile it using :
```bash
rustc oxide.rs -o oxide && ./oxide your-file.oxd
```
Oxide also requires the file format **.oxd**. So, if you compile a non-.oxd file through the Oxide compiler you just made, it **will** give an error.
# Recommendations
* At the moment, there's no syntax highlighting for Oxide. If you use VS Code, however, you can set the syntax highlighting to that of Rust or C++ and it will work almost perfectly!
* Use "println?" most of the time. However, for interactive games where you have to enter data and such, it's recommended to use "print?". This is because "print?" is just "println" but it doesn't add a newline. I know that doesn't sound great, but one day, it will come in handy.
* The sleep function measures in milliseconds, not seconds. So if you type "sleep?(1)" and it loads almost instantaneously, then that's just how the code works. But if you type in "sleep?(1)" and it actually loads in 1 second, then get off the drugs.
* Use variables, kids!
# Notes
*Oxide is at version 0.01.*

So, if there are any bugs, make an issue on this repo and I will probably update it!

*Oxide is being developed being a single person.*

So, it would really help for you to... make an issue again and paste the feature you want in the project, like a lexer. Or even better, a fully developed interpreter that isn't part of rustc or any pre-built programming language such as Rust! Or anything you feel should be added!

*Oxide is a programming language with zero community.*

So, if you somehow stumbled upon this README.md, share it with your friends and tell them to share it to their friends! Or you can make a post on Oxide so other people can see it!
# Upcoming features
There will 100% be in the near future :
* *Library support*
* *Extensive documentation*
* *Multi-line comments*
* *A bit of C & Rust compatibility*
* *Over 100+ things I don't feel like naming right now*
* *And most of the things you guys suggest!*

Alright, that's all, folks!
