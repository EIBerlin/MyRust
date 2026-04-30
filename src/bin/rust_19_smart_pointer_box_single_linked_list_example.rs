#[derive(Debug)]
enum UndoStep {
    Node { step: String, next: Box<UndoStep> },
    Empty,
}

impl UndoStep {
    fn add_step(self, new_command: String) -> UndoStep {
        UndoStep::Node {
            step: new_command,
            next: Box::new(self),
        }
    }

    fn remove_step(self, target_command: &str) -> UndoStep {
        match self {
            UndoStep::Node { step, next } => {
                if step == target_command {
                    *next
                } else {
                    UndoStep::Node {
                        step,
                        next: Box::new(next.remove_step(target_command)),
                    }
                }
            }
            _ => UndoStep::Empty,
        }
    }
}

fn main() {
    let mut history = UndoStep::Empty;

    history = history.add_step(String::from("Step 1"));
    history = history.add_step(String::from("Step 2"));
    history = history.add_step(String::from("Step 3"));

    println!("Primary History: \n{:?}\n", history);

    history = history.remove_step("Step 2");

    println!("After removing step 2: \n{:?}", history);
}
