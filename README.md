# rust_calc
The Rust Calculator project, also known as `rust_calc`, is a graphical calculator application built with the Rust programming language and GTK toolkit.
<br>
It allows users to perform basic arithmetic operations, including addition, subtraction, multiplication, and division.
<br>
The calculator features a clean and intuitive graphical user interface, making it easy for users to input expressions and obtain results.
<br>
This project includes tests to ensure that the calculator performs the correct operations and handles errors appropriately.
<br>
Say NO to 'Legacy code'

## Features

- Addition
- Subtraction
- Multiplication
- Division
- ... and more

### How operations are added to the GUI

```rust
 let operations = vec!["+", "-", "*", "/"];
    for (i, operation) in operations.iter().enumerate() {
        let operation = operation.clone();
        let button = Button::with_label(operation);
        let label_clone = Rc::clone(&label);
        button.connect_clicked(move |_| {
            let current_text = label_clone.borrow().text().to_string();
            let new_text = format!("{} {} ", current_text, operation);
            label_clone.borrow_mut().set_text(&new_text);
        });
        grid.attach(&button, 3, i as i32 + 1, 1, 1);
    }
```

### How operations are performed

```rust
  match parts[1] {
        "+" => Ok(operand1 + operand2),
        "-" => Ok(operand1 - operand2),
        "*" => Ok(operand1 * operand2),
        "/" => {
            if operand2 != 0.0 {
                Ok(operand1 / operand2)
            } else {
                Err("Division by zero")
            }
        }
        _ => Err("Invalid operator"),
    }
```

## How to Run

### Prerequisites
Make sure you have Rust on your machine.

### Download the Unix Executable File
This executable file is located in the releases tab, or you can [download it here](https://github.com/Telmo-Sousa/rust_calc/releases/download/1.1/rust_calc).
<br>
It can be created when you run the project and is found at /src/target/debug or /src/target/release depending on how you build it.
<br>
To check if your system is Unix based [click here](https://en.wikipedia.org/wiki/List_of_Unix_systems).

### Run the Project
To run the project, execute the following command:

```bash
cargo run
```

## Example

![Image](/image.png)

This is a brief example of my project.
