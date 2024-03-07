# Rust
> In this article I will cover what Rust is, its key paradigms, and how it uses OOP (objects). I will also briefly cover how to get started with Rust.
## What is Rust?
- Rust is a systems programming language that is focused on safety, speed, and concurrency.
- It was created by Graydon Hoare at Mozilla Research and first appeared in 2010.
- Rust is designed to be a safe, concurrent, and practical language.
- It is a statically typed language that is designed to be memory safe and thread safe.
- Rust is often compared to C++ and is often seen as a safer alternative to C++.
- Rust is also often compared to Go and is often seen as a more low-level alternative to Go.
- Rust is sometimes criticized for being difficult to learn and for having a steep learning curve. However, many people find that once they get past the initial learning curve, Rust is a very productive and enjoyable language to work with. I certainly found this to be the case when I first started learning Rust.
## Key Paradigms
- As with most modern languages, Rust is multi-paradigm. It supports both functional and imperative programming.
- It is a compiled systems language, which has been widely demonstrated to be as fast if not faster than C, and almost always faster than C++.
- Rust is designed to be a safe language. It has a strong type system and a strong ownership model that prevents many common programming errors.
- It operates on the principle of "zero-cost abstractions", which means that abstractions in Rust do not incur a runtime cost.
- Rust has a strong focus on concurrency. It has a powerful and flexible concurrency model that allows for safe and efficient concurrent programming.
## OOP in Rust
- unlike Java, Rust does not have classes. Instead, it has structs and enums.
- Rust has traits, which are similar to interfaces in Java.
### Structs
- Structs are used to create custom data types.
- They are similar to classes in Java, but they are more flexible and powerful.
- They are syntactically similar to Java's classes and C/C++'s structs.
### Enums
- Enums are used to create custom data types that can have a fixed set of values.
- They are similar to Java's enums, but again, they are more flexible and powerful.
- They are very similar to C/C++'s enums.
### Traits
- Traits are used to define behavior for types.
- They define a strict set of methods that a type must implement in order to be considered to have that behavior.
### Examples
#### Structs
- here is a simple person/programmer example in Java, C++, and Rust:
```java
// Java
public class Person {
    private String name;
    private int age;
    public Person(String name, int age) {
        this.name = name;
        this.age = age;
    }
    public String sayHello() {
        return "Hello, my name is " + name + " and I am " + age + " years old.";
    }
}

public class Programmer extends Person {
    private String language;
    public Programmer(String name, int age, String language) {
        super(name, age);
        this.language = language;
    }
    public String sayHello() {
        return "Hello, my name is " + name + " and I am " + age + " years old. I program in " + language + ".";
    }
}
```
```cpp
// C++
class Person {
    private:
        std::string name;
        int age;
    public:
        Person(std::string name, int age) : name(name), age(age) {}
        std::string sayHello() {
            return "Hello, my name is " + name + " and I am " + std::to_string(age) + " years old.";
        }
};

class Programmer : public Person {
    private:
        std::string language;
    public:
        Programmer(std::string name, int age, std::string language) : Person(name, age), language(language) {}
        std::string sayHello() {
            return "Hello, my name is " + name + " and I am " + std::to_string(age) + " years old. I program in " + language + ".";
        }
};
```
```c
// C
typedef struct {
    char *name;
    int age;
} Person;

char *person_say_hello(Person *person) {
    char *hello = malloc(100);
    sprintf(hello, "Hello, my name is %s and I am %d years old.", person->name, person->age);
    return hello;
}

typedef struct {
    Person person;
    char *language;
} Programmer;

char *programmer_say_hello(Programmer *programmer) {
    char *hello = malloc(100);
    sprintf(hello, "Hello, my name is %s and I am %d years old. I program in %s.", programmer->person.name, programmer->person.age, programmer->language);
    return hello;
}
```
```rust
// Rust
struct Person { // struct PascalCase
    name: String,
    age: u32, // Rust has no built-in int type, only u32, i32, etc (up to 128 bits, and 64 for floating point values)
}

trait SayHello { // trait PascalCase
    fn say_hello(&self) -> String; // fn snake_case
}

impl SayHello for Person {
    fn say_hello(&self) -> String { //
        format!("Hello, my name is {} and I am {} years old.", self.name, self.age)
    }
}

struct Programmer {
    person: Person, // Rust has no inheritance, so we use composition instead
    language: String,
}

impl SayHello for Programmer {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {} and I am {} years old. I program in {}.", self.person.name, self.person.age, self.language)
    }
}
```

#### Enums
- here is a simple color example in Java, C++, and Rust:
```java
// Java
public enum Color {
    RED, GREEN, BLUE;
}

public void printWithUnixColor(String message, Color color) {
    switch (color) {
        case RED:
            System.out.println("\u001B[31m" + message + "\u001B[0m");
            break;
        case GREEN:
            System.out.println("\u001B[32m" + message + "\u001B[0m");
            break;
        case BLUE:
            System.out.println("\u001B[34m" + message + "\u001B[0m");
            break;
    }
}
```
```cpp
// C++
enum class Color {
    RED, GREEN, BLUE
};

void print_with_unix_color(std::string message, Color color) {
    switch (color) {
        case Color::RED:
            std::cout << "\033[31m" << message << "\033[0m" << std::endl;
            break;
        case Color::GREEN:
            std::cout << "\033[32m" << message << "\033[0m" << std::endl;
            break;
        case Color::BLUE:
            std::cout << "\033[34m" << message << "\033[0m" << std::endl;
            break;
    }
}
```
```c
// C
typedef enum {
    RED, GREEN, BLUE
} Color;

void print_with_unix_color(char *message, Color color) {
    switch (color) {
        case RED:
            printf("\033[31m%s\033[0m\n", message);
            break;
        case GREEN:
            printf("\033[32m%s\033[0m\n", message);
            break;
        case BLUE:
            printf("\033[34m%s\033[0m\n", message);
            break;
    }
}
```
```rust
// Rust
enum Color {
    RED, GREEN, BLUE,
}

impl Color { // enums can have methods, and not all functions need to be implemented in traits, for a user defined type a straight impl is allowed
    fn print_with_unix_color(&self, message: &str) {
        match self {
            Color::RED => println!("\u{001B}[31m{}\u{001B}[0m", message),
            Color::GREEN => println!("\u{001B}[32m{}\u{001B}[0m", message),
            Color::BLUE => println!("\u{001B}[34m{}\u{001B}[0m", message),
        }
    }
}
```
## Getting Started
- To get started with Rust, you will need to install the Rust toolchain. You can do this by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
- On Unices, you can run the following command to install Rust:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Once you have installed Rust, you can create a new project by running the following command:
```sh
cargo new my_project
```
- This will create a new directory called `my_project` with a basic Rust project structure.
- You can then navigate into the `my_project` directory and run the following command to build and run your project:
```sh
cargo run
```
- You can then edit the code in the `src/main.rs` file to create your own Rust program.
- You can use modules, third-party libraries, and other features to create more complex Rust programs.