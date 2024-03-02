use gdk;
use gtk::prelude::*;
use gtk::{Button, Grid, Label, Window, WindowType};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("rust_calc by Sousa");
    window.set_default_size(200, 200);

    // Create a grid to organize the buttons
    let grid = Grid::new();
    window.add(&grid);

    // Create a label to display the calculator input and result
    let label = Rc::new(RefCell::new(Label::new(Some(""))));
    let button = Rc::new(RefCell::new(Button::new()));
    button.borrow().add(&*label.borrow());
    grid.attach(&*button.borrow(), 0, 0, 4, 1);

    // Number buttons
    let numbers = vec!["7", "8", "9", "4", "5", "6", "1", "2", "3", "0", "."];
    for (i, number) in numbers.iter().enumerate() {
        let number = number.clone();
        let button = Button::with_label(number);
        let label_clone = Rc::clone(&label);
        button.connect_clicked(move |_| {
            let current_text = label_clone.borrow().text().to_string();
            let new_text = format!("{}{}", current_text, number);
            label_clone.borrow_mut().set_text(&new_text);
        });
        grid.attach(&button, (i % 3) as i32, (i / 3) as i32 + 1, 1, 1);
    }

    // Operation buttons
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

    // Equals button
    let equals_button = Button::with_label("=");
    let label_clone = Rc::clone(&label);
    equals_button.connect_clicked(move |_| {
        let expression = label_clone.borrow().text().to_string();
        if let Ok(result) = eval_expression(&expression) {
            label_clone.borrow_mut().set_text(&result.to_string());
        } else {
            label_clone.borrow_mut().set_text("Error");
        }
    });
    grid.attach(&equals_button, 2, 4, 1, 1);

    // Clear button
    let clear_button = Button::with_label("Clear");
    let label_clone = Rc::clone(&label);
    clear_button.connect_clicked(move |_| {
        label_clone.borrow_mut().set_text("");
    });
    grid.attach(&clear_button, 0, 5, 2, 1);

    // Copy result button
    let copy_button = Button::with_label("Copy");
    let label_clone = Rc::clone(&label);
    copy_button.connect_clicked(move |_| {
        let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
        let text = label_clone.borrow().text().to_string();
        clipboard.set_text(&text);
    });
    grid.attach(&copy_button, 0, 6, 2, 1);

    // Backspace button
    let backspace_button = Button::with_label("⌫");
    let label_clone = Rc::clone(&label);
    backspace_button.connect_clicked(move |_| {
        let current_text = label_clone.borrow().text().to_string();
        if current_text.is_empty() {
            return;
        }
        let new_text = current_text[..current_text.len() - 1].to_string();
        label_clone.borrow_mut().set_text(&new_text);
    });
    grid.attach(&backspace_button, 2, 5, 2, 1);

    // Paste button
    let paste_button = Button::with_label("Paste");
    let label_clone = Rc::clone(&label);
    paste_button.connect_clicked(move |_| {
        let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
        if let Some(text) = clipboard.wait_for_text() {
            let current_text = label_clone.borrow().text().to_string();
            let new_text = format!("{}{}", current_text, text);
            label_clone.borrow_mut().set_text(&new_text);
        }
    });
    grid.attach(&paste_button, 2, 6, 2, 1);

    // Handle the destroy event to exit the application
    window.connect_destroy(|_| {
        gtk::main_quit();
    });

    // Show all components
    window.show_all();

    // Start the GTK main loop
    gtk::main();
}

fn eval_expression(expression: &str) -> Result<f64, &'static str> {
    // Split the expression into operands and operator
    let parts: Vec<&str> = expression.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid expression");
    }

    // Parse operands
    let operand1: f64 = parts[0].parse().map_err(|_| "Invalid operand")?;
    let operand2: f64 = parts[2].parse().map_err(|_| "Invalid operand")?;

    // Perform the operation
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
}
