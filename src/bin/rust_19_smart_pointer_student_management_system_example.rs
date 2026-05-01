/**
 * - We will create this with rust, smart_pointer, box, single linked list
 *
 * Student Management System
 *
 * This Student Management System manages students, classes, and attendance records.
 * Each student has an ID, name, and daily attendance history using optional values.
 * Classes have a start date, month, and multiple students.
 * The system supports adding and finding students, recording attendance,
 * and calculating attendance by month, from start date to current date, and yearly averages.
 *
 * Entities:
 * - Student:
 *   - id: unique identifier
 *   - name: student name
 *   - attendance_history: daily attendance records (Option<bool> per day)
 *     - Some(true): present
 *     - Some(false): absent
 *     - None: no record
 *   - class_ids: list of class IDs the student belongs to
 *
 * - Class:
 *   - id: unique identifier
 *   - name: class name
 *   - month: month used for attendance tracking (e.g., "2026-04")
 *   - start_date: class start date
 *   - student_ids: list of student IDs in the class
 *
 * Relationships:
 * - Many-to-many:
 *   - A class can have many students
 *   - A student can belong to many classes
 *
 * Features (Implementation Requirements):
 * - add_student: add a new student to the system
 * - find_student: retrieve a student by ID
 * - add_attendance: record attendance for a specific day
 * - get_attendance: retrieve attendance history for a student
 * - set_class_start_date: define or update class start date
 * - calculate_monthly_attendance: compute attendance for a given month
 * - calculate_attendance: compute attendance from the class start date up to the current date
 * - calculate_yearly_average_attendance: compute average attendance per year
 */

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Self { value, next }
    }
}

struct AttendanceRecord {
    date: String,
    status: Option<bool>,
}

struct Student {
    id: u8,
    name: String,
    attendance_history: Option<Box<Node<AttendanceRecord>>>,
}

struct Class {
    id: u8,
    name: String,
    start_date: String,
    student_ids: Option<Box<Node<u8>>>,
}

struct System {
    students: Option<Box<Node<Student>>>,
    classes: Option<Box<Node<Class>>>,
}

impl System {
    fn new() -> Self {
        Self {
            students: None,
            classes: None,
        }
    }

    fn add_class(&mut self, id: u8, name: String, start_date: String) -> &mut Self {
        let class = Class {
            id,
            name,
            start_date,
            student_ids: None,
        };

        self.classes = Some(Box::new(Node::new(class, self.classes.take())));
        self
    }

    fn add_student(&mut self, id: u8, name: String, class_id: u8) -> &mut Self {
        let mut cur = self.classes.as_deref_mut();
        while let Some(node) = cur {
            if node.value.id == class_id {
                node.value.student_ids =
                    Some(Box::new(Node::new(id, node.value.student_ids.take())));
                break;
            }
            cur = node.next.as_deref_mut();
        }

        let student = Student {
            id,
            name,
            attendance_history: None,
        };

        self.students = Some(Box::new(Node::new(student, self.students.take())));
        self
    }

    fn add_attendance(&mut self, student_id: u8, date: String, status: Option<bool>) -> &mut Self {
        let mut cur = self.students.as_deref_mut();

        while let Some(node) = cur {
            if node.value.id == student_id {
                node.value.attendance_history = Some(Box::new(Node::new(
                    AttendanceRecord { date, status },
                    node.value.attendance_history.take(),
                )));
                break;
            }
            cur = node.next.as_deref_mut();
        }

        self
    }

    fn find_student(&mut self, id: u8) -> &mut Self {
        let mut cur = self.students.as_deref();

        while let Some(node) = cur {
            if node.value.id == id {
                println!("Found: {} - {}", node.value.id, node.value.name);
                return self;
            }
            cur = node.next.as_deref();
        }

        println!("Student {} not found", id);
        self
    }

    fn get_attendance(&mut self, student_id: u8) -> &mut Self {
        println!("Attendance for {}", student_id);

        let mut cur = self.students.as_deref();

        while let Some(node) = cur {
            if node.value.id == student_id {
                let mut att = node.value.attendance_history.as_deref();

                if att.is_none() {
                    println!("  No records");
                    return self;
                }

                while let Some(a) = att {
                    let status = match a.value.status {
                        Some(true) => "Present",
                        Some(false) => "Absent",
                        None => "No Record",
                    };

                    println!("  {} -> {}", a.value.date, status);
                    att = a.next.as_deref();
                }

                return self;
            }

            cur = node.next.as_deref();
        }

        println!("  Student not found");
        self
    }

    fn calculate_monthly_attendance(&mut self, student_id: u8, month: &str) -> &mut Self {
        let (present, total) = self.count_attendance(student_id, |d| d.starts_with(month));
        self.print_percentage(&format!("Monthly {}", month), present, total)
    }

    fn calculate_attendance(&mut self, student_id: u8, class_id: u8) -> &mut Self {
        let mut start_date = "";

        let mut cur_class = self.classes.as_deref();
        while let Some(node) = cur_class {
            if node.value.id == class_id {
                start_date = &node.value.start_date;
                break;
            }
            cur_class = node.next.as_deref();
        }

        let today = "2026-05-01";

        let (present, total) = self.count_attendance(student_id, |d| d >= start_date && d <= today);

        self.print_percentage("From Start", present, total)
    }

    fn calculate_yearly_average_attendance(&mut self, student_id: u8) -> &mut Self {
        let (present, total) = self.count_attendance(student_id, |_| true);
        self.print_percentage("Yearly Avg", present, total)
    }

    fn finish(&mut self) -> &mut Self {
        println!("=== Done ===");
        self
    }

    fn count_attendance<F>(&self, student_id: u8, filter: F) -> (u32, u32)
    where
        F: Fn(&str) -> bool,
    {
        let mut present = 0;
        let mut total = 0;

        let mut cur = self.students.as_deref();

        while let Some(node) = cur {
            if node.value.id == student_id {
                let mut att = node.value.attendance_history.as_deref();

                while let Some(a) = att {
                    if filter(&a.value.date) {
                        if let Some(status) = a.value.status {
                            total += 1;
                            if status {
                                present += 1;
                            }
                        }
                    }
                    att = a.next.as_deref();
                }

                break;
            }

            cur = node.next.as_deref();
        }

        (present, total)
    }

    fn print_percentage(&mut self, label: &str, present: u32, total: u32) -> &mut Self {
        let pct = if total == 0 {
            0.0
        } else {
            present as f32 / total as f32 * 100.0
        };

        println!("{}: {:.1}%", label, pct);
        self
    }
}

fn main() {



    let mut system = System::new();

    system
        .add_class(1, "Math".to_string(), "2026-04-01".to_string())
        .add_class(2, "Physics".to_string(), "2026-04-01".to_string())
        .add_class(3, "Science".to_string(), "2026-04-01".to_string())
        .add_student(1, "Z".to_string(), 1)
        .add_student(2, "EH".to_string(), 1)
        .add_attendance(1, "2026-04-01".to_string(), Some(true))
        .add_attendance(2, "2026-04-01".to_string(), Some(true))
        .find_student(1)
        .find_student(2)
        .get_attendance(1)
        .get_attendance(2)
        .calculate_monthly_attendance(1, "2026-04")
        .calculate_attendance(1, 1)
        .calculate_yearly_average_attendance(1)
        .finish();
}
