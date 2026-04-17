# OMNIPEDIA VOLUME 5: UNIVERSAL TEACHING PATTERNS
# How the AI should explain ANY language to a user

---


## Universal Pattern: Hello World

The AI must recognize this pattern across 47 languages:

### C
```
#include <stdio.h>
int main() { printf("Hello, World!\n"); return 0; }
```

### C++
```
#include <iostream>
int main() { std::cout << "Hello, World!" << std::endl; }
```

### Python
```
print("Hello, World!")
```

### Java
```
public class Hello { public static void main(String[] args) { System.out.println("Hello, World!"); } }
```

### JavaScript
```
console.log("Hello, World!");
```

### Rust
```
fn main() { println!("Hello, World!"); }
```

### Go
```
package main
import "fmt"
func main() { fmt.Println("Hello, World!") }
```

### Ruby
```
puts "Hello, World!"
```

### Swift
```
print("Hello, World!")
```

### Kotlin
```
fun main() = println("Hello, World!")
```

### Haskell
```
main = putStrLn "Hello, World!"
```

### Elixir
```
IO.puts "Hello, World!"
```

### Lua
```
print("Hello, World!")
```

### PHP
```
<?php echo "Hello, World!"; ?>
```

### Perl
```
print "Hello, World!\n";
```

### R
```
cat("Hello, World!\n")
```

### Scala
```
println("Hello, World!")
```

### Dart
```
void main() { print("Hello, World!"); }
```

### TypeScript
```
console.log("Hello, World!");
```

### C#
```
Console.WriteLine("Hello, World!");
```

### F#
```
printfn "Hello, World!"
```

### OCaml
```
print_endline "Hello, World!"
```

### Erlang
```
io:format("Hello, World!~n").
```

### Clojure
```
(println "Hello, World!")
```

### Julia
```
println("Hello, World!")
```

### Nim
```
echo "Hello, World!"
```

### Crystal
```
puts "Hello, World!"
```

### V
```
println("Hello, World!")
```

### Zig
```
const std = @import("std");
pub fn main() void { std.debug.print("Hello, World!\n", .{}); }
```

### FORTRAN
```
      PROGRAM HELLO
      PRINT *, 'Hello, World!'
      END
```

### COBOL
```
       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO.
       PROCEDURE DIVISION.
           DISPLAY 'Hello, World!'.
           STOP RUN.
```

### Pascal
```
program Hello;
begin
  WriteLn('Hello, World!');
end.
```

### Ada
```
with Ada.Text_IO; use Ada.Text_IO;
procedure Hello is begin Put_Line ("Hello, World!"); end Hello;
```

### Prolog
```
:- write('Hello, World!'), nl.
```

### Lisp
```
(format t "Hello, World!~%")
```

### Scheme
```
(display "Hello, World!") (newline)
```

### Brainfuck
```
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

### Assembly x86
```
section .data
    msg db "Hello, World!", 10
section .text
    global _start
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, 14
    syscall
    mov rax, 60
    xor rdi, rdi
    syscall
```

### SQL
```
SELECT 'Hello, World!';
```

### Bash
```
echo "Hello, World!"
```

### PowerShell
```
Write-Host "Hello, World!"
```

### MATLAB
```
disp('Hello, World!')
```

### Wolfram
```
Print["Hello, World!"]
```

### APL
```
'Hello, World!'
```

### Solidity
```
pragma solidity ^0.8.0;
contract Hello { function hello() public pure returns (string memory) { return "Hello, World!"; } }
```

### GDScript
```
func _ready():
    print("Hello, World!")
```

### Mojo
```
fn main():
    print("Hello, World!")
```


## Universal Pattern: Variables

The AI must recognize this pattern across 10 languages:

### C
```
int x = 42;
float pi = 3.14;
char ch = 'A';
const int MAX = 100;
```

### Python
```
x = 42
pi = 3.14
name = 'Ali'
MAX = 100  # convention
```

### JavaScript
```
let x = 42;
const PI = 3.14;
var name = 'Ali';
let items = [1, 2, 3];
```

### Rust
```
let x = 42;
let mut y = 0;
let pi: f64 = 3.14;
const MAX: i32 = 100;
```

### Go
```
x := 42
var pi float64 = 3.14
const MAX = 100
```

### Java
```
int x = 42;
final double PI = 3.14;
String name = "Ali";
var items = List.of(1, 2, 3);
```

### Kotlin
```
val x = 42  // immutable
var y = 0   // mutable
const val MAX = 100
```

### Swift
```
let x = 42  // immutable
var y = 0   // mutable
let pi: Double = 3.14
```

### Haskell
```
x = 42  -- all immutable
pi = 3.14
name = "Ali"
```

### Ruby
```
x = 42
pi = 3.14
MAX = 100  # constant
@instance_var = 'hello'
@@class_var = 'world'
```


## Universal Pattern: Functions

The AI must recognize this pattern across 23 languages:

### C
```
int add(int a, int b) { return a + b; }
```

### Python
```
def add(a, b):
    return a + b

add_lambda = lambda a, b: a + b
```

### JavaScript
```
function add(a, b) { return a + b; }
const add2 = (a, b) => a + b;
```

### Rust
```
fn add(a: i32, b: i32) -> i32 { a + b }
```

### Go
```
func add(a, b int) int { return a + b }
```

### Java
```
public int add(int a, int b) { return a + b; }
```

### Kotlin
```
fun add(a: Int, b: Int): Int = a + b
```

### Swift
```
func add(_ a: Int, _ b: Int) -> Int { a + b }
```

### Haskell
```
add :: Int -> Int -> Int
add a b = a + b
```

### Ruby
```
def add(a, b)
  a + b
end
```

### Elixir
```
def add(a, b), do: a + b
```

### Lua
```
function add(a, b) return a + b end
```

### PHP
```
function add(int $a, int $b): int { return $a + $b; }
```

### Scala
```
def add(a: Int, b: Int): Int = a + b
```

### Julia
```
add(a, b) = a + b
```

### Nim
```
proc add(a, b: int): int = a + b
```

### Crystal
```
def add(a : Int32, b : Int32) : Int32
  a + b
end
```

### Dart
```
int add(int a, int b) => a + b;
```

### C#
```
int Add(int a, int b) => a + b;
```

### F#
```
let add a b = a + b
```

### OCaml
```
let add a b = a + b
```

### Erlang
```
add(A, B) -> A + B.
```

### Clojure
```
(defn add [a b] (+ a b))
```


## Universal Pattern: Loops

The AI must recognize this pattern across 9 languages:

### C
```
for (int i = 0; i < 10; i++) { printf("%d\n", i); }
```

### Python
```
for i in range(10):
    print(i)

while condition:
    do_something()
```

### JavaScript
```
for (let i = 0; i < 10; i++) { console.log(i); }
for (const item of array) { console.log(item); }
```

### Rust
```
for i in 0..10 { println!("{}", i); }
loop { break; }
```

### Go
```
for i := 0; i < 10; i++ { fmt.Println(i) }
for _, v := range slice { fmt.Println(v) }
```

### Ruby
```
10.times { |i| puts i }
(1..10).each { |n| puts n }
array.each_with_index { |val, idx| puts val }
```

### Haskell
```
-- No traditional loops, use recursion and map
map (\x -> x * 2) [1..10]
forM_ [1..10] print
```

### Kotlin
```
for (i in 0 until 10) { println(i) }
for ((index, value) in list.withIndex()) { }
```

### Swift
```
for i in 0..<10 { print(i) }
for item in array { print(item) }
```


## Universal Pattern: Error Handling

The AI must recognize this pattern across 9 languages:

### C
```
if (ptr == NULL) { perror("Error"); exit(1); }
errno = 0; result = operation(); if (errno) { handle_error(); }
```

### Python
```
try:
    result = risky_operation()
except ValueError as e:
    print(f'Error: {e}')
except Exception as e:
    print(f'Unexpected: {e}')
finally:
    cleanup()
```

### JavaScript
```
try {
    const result = await riskyOperation();
} catch (error) {
    console.error('Failed:', error.message);
} finally {
    cleanup();
}
```

### Rust
```
match std::fs::read_to_string("file.txt") {
    Ok(content) => println!("{}", content),
    Err(e) => eprintln!("Error: {}", e),
}
// Or with ? operator:
let content = std::fs::read_to_string("file.txt")?;
```

### Go
```
result, err := doSomething()
if err != nil {
    log.Fatalf("Error: %v", err)
}
```

### Java
```
try {
    result = riskyOperation();
} catch (IOException e) {
    logger.error("IO error", e);
} catch (Exception e) {
    logger.error("General error", e);
} finally {
    resource.close();
}
```

### Haskell
```
case safeDivide x y of
  Nothing -> putStrLn "Division by zero"
  Just result -> print result
```

### Elixir
```
case File.read("file.txt") do
  {:ok, content} -> IO.puts(content)
  {:error, reason} -> IO.puts("Error: #{reason}")
end
```

### Swift
```
do {
    let data = try fetchData()
    process(data)
} catch NetworkError.timeout {
    print("Timeout!")
} catch {
    print("Error: \(error)")
}
```

