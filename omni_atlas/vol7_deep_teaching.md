# OMNIPEDIA VOLUME 7: DEEP CROSS-LANGUAGE TEACHING

---


======================================================================
## TOPIC: Object-Oriented Programming
======================================================================

The AI must demonstrate this concept in 11 languages:

### Python
```python
class Animal:
    def __init__(self, name):
        self.name = name
    def speak(self):
        raise NotImplementedError

class Dog(Animal):
    def speak(self):
        return f'{self.name} says Woof!'

class Cat(Animal):
    def speak(self):
        return f'{self.name} says Meow!'

animals = [Dog('Rex'), Cat('Whiskers')]
for a in animals:
    print(a.speak())
```

### Java
```java
abstract class Animal {
    protected String name;
    public Animal(String name) { this.name = name; }
    public abstract String speak();
}

class Dog extends Animal {
    public Dog(String name) { super(name); }
    public String speak() { return name + " says Woof!"; }
}

class Cat extends Animal {
    public Cat(String name) { super(name); }
    public String speak() { return name + " says Meow!"; }
}

List<Animal> animals = List.of(new Dog("Rex"), new Cat("Whiskers"));
animals.forEach(a -> System.out.println(a.speak()));
```

### C++
```c++
class Animal {
protected:
    std::string name;
public:
    Animal(std::string n) : name(std::move(n)) {}
    virtual ~Animal() = default;
    virtual std::string speak() const = 0;
};

class Dog : public Animal {
public:
    using Animal::Animal;
    std::string speak() const override { return name + " says Woof!"; }
};

std::vector<std::unique_ptr<Animal>> animals;
animals.push_back(std::make_unique<Dog>("Rex"));
```

### Rust
```rust
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
}

struct Dog { name: String }
impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) -> String { format!("{} says Woof!", self.name) }
}

let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog { name: "Rex".into() })];
```

### Go
```go
type Animal interface {
    Speak() string
}

type Dog struct{ Name string }
func (d Dog) Speak() string { return d.Name + " says Woof!" }

type Cat struct{ Name string }
func (c Cat) Speak() string { return c.Name + " says Meow!" }

animals := []Animal{Dog{"Rex"}, Cat{"Whiskers"}}
for _, a := range animals { fmt.Println(a.Speak()) }
```

### Swift
```swift
protocol Animal {
    var name: String { get }
    func speak() -> String
}

struct Dog: Animal {
    let name: String
    func speak() -> String { "\(name) says Woof!" }
}

let animals: [Animal] = [Dog(name: "Rex")]
animals.forEach { print($0.speak()) }
```

### Kotlin
```kotlin
abstract class Animal(val name: String) {
    abstract fun speak(): String
}

class Dog(name: String) : Animal(name) {
    override fun speak() = "$name says Woof!"
}

val animals = listOf(Dog("Rex"))
animals.forEach { println(it.speak()) }
```

### TypeScript
```typescript
interface Animal {
    name: string;
    speak(): string;
}

class Dog implements Animal {
    constructor(public name: string) {}
    speak() { return `${this.name} says Woof!`; }
}

const animals: Animal[] = [new Dog('Rex')];
animals.forEach(a => console.log(a.speak()));
```

### Ruby
```ruby
class Animal
  attr_reader :name
  def initialize(name) = @name = name
  def speak = raise NotImplementedError
end

class Dog < Animal
  def speak = "#{name} says Woof!"
end

[Dog.new('Rex')].each { |a| puts a.speak }
```

### C#
```c#
abstract class Animal {
    public string Name { get; }
    protected Animal(string name) => Name = name;
    public abstract string Speak();
}

class Dog : Animal {
    public Dog(string name) : base(name) {}
    public override string Speak() => $"{Name} says Woof!";
}

var animals = new List<Animal> { new Dog("Rex") };
animals.ForEach(a => Console.WriteLine(a.Speak()));
```

### Scala
```scala
trait Animal {
  def name: String
  def speak: String
}

case class Dog(name: String) extends Animal {
  def speak = s"$name says Woof!"
}

val animals: List[Animal] = List(Dog("Rex"))
animals.foreach(a => println(a.speak))
```


======================================================================
## TOPIC: Concurrency
======================================================================

The AI must demonstrate this concept in 8 languages:

### Go
```go
func worker(id int, jobs <-chan int, results chan<- int) {
    for j := range jobs {
        fmt.Printf("Worker %d processing job %d\n", id, j)
        time.Sleep(time.Second)
        results <- j * 2
    }
}

func main() {
    jobs := make(chan int, 100)
    results := make(chan int, 100)
    for w := 1; w <= 3; w++ { go worker(w, jobs, results) }
    for j := 1; j <= 5; j++ { jobs <- j }
    close(jobs)
    for a := 1; a <= 5; a++ { fmt.Println(<-results) }
}
```

### Rust
```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i * 2).unwrap();
        });
    }
    drop(tx);
    for received in rx {
        println!("Got: {}", received);
    }
}
```

### Python
```python
import asyncio

async def worker(name, queue):
    while True:
        item = await queue.get()
        print(f'{name} processing {item}')
        await asyncio.sleep(1)
        queue.task_done()

async def main():
    queue = asyncio.Queue()
    for i in range(10): queue.put_nowait(i)
    tasks = [asyncio.create_task(worker(f'w-{i}', queue)) for i in range(3)]
    await queue.join()
    for t in tasks: t.cancel()

asyncio.run(main())
```

### Erlang
```erlang
-module(worker).
-export([start/0, loop/0]).

start() ->
    Pids = [spawn(fun loop/0) || _ <- lists:seq(1, 3)],
    [Pid ! {work, N} || N <- lists:seq(1, 10), Pid <- Pids],
    ok.

loop() ->
    receive
        {work, N} ->
            io:format("Processing ~p~n", [N]),
            loop()
    end.
```

### Elixir
```elixir
defmodule Worker do
  def start do
    1..10
    |> Enum.map(&Task.async(fn -> process(&1) end))
    |> Enum.map(&Task.await/1)
  end

  defp process(n) do
    :timer.sleep(100)
    IO.puts("Processed #{n}")
    n * 2
  end
end
```

### Kotlin
```kotlin
import kotlinx.coroutines.*

suspend fun worker(id: Int, channel: Channel<Int>) {
    for (item in channel) {
        println("Worker $id processing $item")
        delay(1000)
    }
}

fun main() = runBlocking {
    val channel = Channel<Int>()
    repeat(3) { launch { worker(it, channel) } }
    for (i in 1..10) { channel.send(i) }
    channel.close()
}
```

### Java
```java
ExecutorService executor = Executors.newFixedThreadPool(3);
List<Future<Integer>> futures = new ArrayList<>();
for (int i = 0; i < 10; i++) {
    final int n = i;
    futures.add(executor.submit(() -> {
        Thread.sleep(100);
        System.out.println("Processing " + n);
        return n * 2;
    }));
}
for (Future<Integer> f : futures) System.out.println(f.get());
executor.shutdown();
```

### Swift
```swift
func process(_ n: Int) async -> Int {
    try? await Task.sleep(nanoseconds: 100_000_000)
    print("Processing \(n)")
    return n * 2
}

let results = await withTaskGroup(of: Int.self) { group in
    for i in 1...10 {
        group.addTask { await process(i) }
    }
    var collected = [Int]()
    for await result in group { collected.append(result) }
    return collected
}
```


======================================================================
## TOPIC: Data Structures
======================================================================

The AI must demonstrate this concept in 5 languages:

### Python
```python
# Linked List
class Node:
    def __init__(self, val, next=None):
        self.val = val
        self.next = next

def to_list(head):
    result = []
    while head:
        result.append(head.val)
        head = head.next
    return result

# Binary Tree
class TreeNode:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def inorder(node):
    if not node: return []
    return inorder(node.left) + [node.val] + inorder(node.right)

# Hash Map usage
from collections import defaultdict, Counter
word_count = Counter('hello world hello'.split())
print(word_count)  # Counter({'hello': 2, 'world': 1})
```

### Rust
```rust
// Linked List
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// Binary Tree
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

// HashMap
use std::collections::HashMap;
let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert("Alice".to_string(), 100);
scores.insert("Bob".to_string(), 85);
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
```

### C
```c
// Linked List
typedef struct Node {
    int data;
    struct Node* next;
} Node;

Node* create_node(int data) {
    Node* n = malloc(sizeof(Node));
    n->data = data;
    n->next = NULL;
    return n;
}

// Binary Tree
typedef struct Tree {
    int val;
    struct Tree* left;
    struct Tree* right;
} Tree;

void inorder(Tree* root) {
    if (!root) return;
    inorder(root->left);
    printf("%d ", root->val);
    inorder(root->right);
}
```

### Java
```java
// Linked List
class ListNode<T> {
    T val;
    ListNode<T> next;
    ListNode(T val) { this.val = val; }
}

// Binary Tree
class TreeNode<T> {
    T val;
    TreeNode<T> left, right;
    TreeNode(T val) { this.val = val; }
}

// Collections
Map<String, Integer> scores = new HashMap<>();
scores.put("Alice", 100);
scores.put("Bob", 85);
scores.forEach((k, v) -> System.out.println(k + ": " + v));

List<Integer> sorted = scores.values().stream().sorted().collect(Collectors.toList());
```

### Haskell
```haskell
-- Linked List (built into language as [a])
data List a = Nil | Cons a (List a)

-- Binary Tree
data Tree a = Leaf | Node (Tree a) a (Tree a)

inorder :: Tree a -> [a]
inorder Leaf = []
inorder (Node l v r) = inorder l ++ [v] ++ inorder r

-- Map
import qualified Data.Map as Map
scores = Map.fromList [("Alice", 100), ("Bob", 85)]
Map.lookup "Alice" scores  -- Just 100
```


======================================================================
## TOPIC: Pattern Matching
======================================================================

The AI must demonstrate this concept in 9 languages:

### Rust
```rust
enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    Color(u8, u8, u8),
}

fn process(cmd: Command) {
    match cmd {
        Command::Quit => println!("Quitting"),
        Command::Echo(msg) => println!("{}", msg),
        Command::Move { x, y } => println!("Move to ({}, {})", x, y),
        Command::Color(r, g, b) => println!("Color: #{:02x}{:02x}{:02x}", r, g, b),
    }
}
```

### Haskell
```haskell
data Shape = Circle Double | Rectangle Double Double | Triangle Double Double Double

area :: Shape -> Double
area (Circle r) = pi * r * r
area (Rectangle w h) = w * h
area (Triangle a b c) = let s = (a+b+c)/2 in sqrt(s*(s-a)*(s-b)*(s-c))

describe :: Shape -> String
describe (Circle r) | r > 10 = "Big circle"
                    | otherwise = "Small circle"
describe _ = "Some shape"
```

### Elixir
```elixir
defmodule Shape do
  def area({:circle, r}), do: :math.pi() * r * r
  def area({:rectangle, w, h}), do: w * h
  def area({:triangle, a, b, c}) do
    s = (a + b + c) / 2
    :math.sqrt(s * (s - a) * (s - b) * (s - c))
  end
end

case result do
  {:ok, value} -> IO.puts("Success: #{value}")
  {:error, reason} -> IO.puts("Error: #{reason}")
end
```

### Scala
```scala
sealed trait Shape
case class Circle(r: Double) extends Shape
case class Rectangle(w: Double, h: Double) extends Shape

def area(s: Shape): Double = s match {
  case Circle(r) => Math.PI * r * r
  case Rectangle(w, h) => w * h
}
```

### Kotlin
```kotlin
sealed class Result<out T>
data class Success<T>(val value: T) : Result<T>()
data class Failure(val error: String) : Result<Nothing>()

fun handle(result: Result<Int>) = when (result) {
    is Success -> println("Got: ${result.value}")
    is Failure -> println("Error: ${result.error}")
}
```

### Swift
```swift
enum Shape {
    case circle(radius: Double)
    case rectangle(width: Double, height: Double)
}

func area(_ shape: Shape) -> Double {
    switch shape {
    case .circle(let r): return .pi * r * r
    case .rectangle(let w, let h): return w * h
    }
}
```

### OCaml
```ocaml
type shape =
  | Circle of float
  | Rectangle of float * float

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
```

### F#
```f#
type Shape =
    | Circle of float
    | Rectangle of float * float

let area = function
    | Circle r -> System.Math.PI * r * r
    | Rectangle (w, h) -> w * h
```

### C# 11
```c# 11
string Classify(object obj) => obj switch
{
    int n when n > 0 => "Positive",
    int n when n < 0 => "Negative",
    0 => "Zero",
    string s => $"String({s.Length})",
    null => "Null",
    _ => "Unknown"
};
```


======================================================================
## TOPIC: Sorting Algorithms
======================================================================

The AI must demonstrate this concept in 5 languages:

### Python
```python
# Quick Sort
def quicksort(arr):
    if len(arr) <= 1: return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    mid = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + mid + quicksort(right)

# Merge Sort
def mergesort(arr):
    if len(arr) <= 1: return arr
    mid = len(arr) // 2
    left = mergesort(arr[:mid])
    right = mergesort(arr[mid:])
    return merge(left, right)

def merge(a, b):
    result = []
    i = j = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]: result.append(a[i]); i += 1
        else: result.append(b[j]); j += 1
    result.extend(a[i:])
    result.extend(b[j:])
    return result
```

### Rust
```rust
fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 { return; }
    let pivot = partition(arr);
    quicksort(&mut arr[..pivot]);
    quicksort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}
```

### C
```c
void quicksort(int arr[], int low, int high) {
    if (low < high) {
        int pi = partition(arr, low, high);
        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}

int partition(int arr[], int low, int high) {
    int pivot = arr[high];
    int i = low - 1;
    for (int j = low; j < high; j++) {
        if (arr[j] < pivot) {
            i++;
            int temp = arr[i]; arr[i] = arr[j]; arr[j] = temp;
        }
    }
    int temp = arr[i+1]; arr[i+1] = arr[high]; arr[high] = temp;
    return i + 1;
}
```

### Haskell
```haskell
quicksort :: Ord a => [a] -> [a]
quicksort [] = []
quicksort (x:xs) = quicksort smaller ++ [x] ++ quicksort bigger
  where smaller = [a | a <- xs, a <= x]
        bigger  = [a | a <- xs, a > x]

mergesort :: Ord a => [a] -> [a]
mergesort [] = []
mergesort [x] = [x]
mergesort xs = merge (mergesort left) (mergesort right)
  where (left, right) = splitAt (length xs `div` 2) xs
        merge [] ys = ys
        merge xs [] = xs
        merge (x:xs) (y:ys)
          | x <= y   = x : merge xs (y:ys)
          | otherwise = y : merge (x:xs) ys
```

### Go
```go
func quickSort(arr []int) []int {
    if len(arr) <= 1 { return arr }
    pivot := arr[0]
    var left, right []int
    for _, v := range arr[1:] {
        if v <= pivot { left = append(left, v) } else { right = append(right, v) }
    }
    result := quickSort(left)
    result = append(result, pivot)
    result = append(result, quickSort(right)...)
    return result
}
```

