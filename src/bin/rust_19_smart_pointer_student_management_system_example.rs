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
use std::fmt::Debug;

trait PrintTrait {
    fn one(self) -> Self;
    fn two(self) -> Self;
    fn three(self) -> Self;
    fn get(self);
}
#[derive(Debug)]
struct PrintStruct<T> {
    my_vec: Vec<T>,
}

impl<T: Debug> PrintTrait for PrintStruct<T>
where
    T: From<u8>,
{
    fn one(mut self) -> Self {
        self.my_vec.push(T::from(1));
        self
    }
    fn two(mut self) -> Self {
        self.my_vec.push(T::from(2));
        self
    }
    fn three(mut self) -> Self {
        self.my_vec.push(T::from(3));
        self
    }
    fn get(self) {
        println!("{:?}", self);
    }
}

pub fn main() {
    let my_vec: Vec<u8> = vec![];
    PrintStruct { my_vec }.one().two().three().get();

    let my_vec: Vec<u8> = vec![];
    (PrintStruct { my_vec }).one().three().three().two().get();
}
