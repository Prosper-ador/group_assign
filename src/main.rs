//Registration
// student registration
// group registration
//Shuffling
//group
// label topic
// number of members should be balanced based on the total effective of the class
//students
// names
// group of students 
// cannot be in more than one group
//Save state to file
// file should only be readable by the main program
//The user will enter the names of students, topics, while the code calculates the number of students
// The program should be capable of asking the user depending on the number of topics he entered how it should be partitioned
// The program should be able to save the data to a file
// The program should be able to load the data from a file
// The program should be able to display the data on the screen
// Criteria: -We shouldn't have more topics than students. -The program should have atleast one student and one topic. -
use application::Application;
fn main() {
    println!("Good Morning!");
    let mut app = Application::new();
    app.run();
}
mod data_collection;
mod enums;
mod models;
mod application;
mod traits;