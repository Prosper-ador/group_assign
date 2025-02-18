use std::{collections::HashSet, fmt::Debug};
use crate::{data_collection::DataCollection, enums::labelling::Labelling, models::{group::Group, student::Student, topic::Topic}, traits::{collect::Collect, gen_data_id::GenDataId}};
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Appstate{
    //date: u64,
    labelling: Labelling,
    groups: Vec<Group>,//Rc<Vec<Group>>,  
    students: Vec<Student>,
    topics: Vec<Topic>,
}

#[derive(Debug)]
pub struct Application {
    state: Appstate,
}
impl Application {
    pub fn new() -> Self {
        Self { 
            state: Appstate {
             //date: 0, 
             labelling: Labelling::Numeric,
             groups: Vec::new(), //Rc::new(Vec::new()),
             students: Vec::new(),
             topics: Vec::new(),
            },
        }
    }
    
    pub fn run(&mut self) {
        'a: loop {
            //let mut topic = Topic::collect();
            //
            //topic.set_id(self.state.topics.len() as u32);
            //println!("{:?}", topic);
            //self.state.topics.push(topic);
            Self::collect_gen_data(&mut self.state.topics);
            
            if Self::should_break0() {
                break 'a;   
            }
        }
        
        'b: loop {
            //let mut student = Student::collect();
            //
            //student.set_id(self.state.students.len() as u32);
            //println!("{:?}", student);
            //self.state.students.push(student);
            Self::collect_gen_data(&mut self.state.students);
            
            
            if Self::should_break1() {
                break 'b;
            }
        }
        self.gen_groups();
    }

    fn gen_groups(&mut self) {
        use rand::rng;
        
        let Appstate { 
            topics,
            students,
            labelling,
            .. 
        } = &self.state;
        let mut new_groups = Vec::new();
        let nbr_of_members = students.len() / topics.len();
        //let nbr_of_members = self.state.students.len() / self.state.topics.len();
        let mut assigned_student_ids = HashSet::<u32>::new();
        
        for topic in topics { //&self.state.topics
            let current_group_id = new_groups.len() +1;
            let label = Self::label_gen(labelling.to_owned(), current_group_id); //self.state.labelling, groups.len()

            let mut students: Vec<Student> = self
                .state
                .students
                .iter()
                .filter(|s| !assigned_student_ids.contains(&s.get_id()))
                .map(|s|s.to_owned())
                .collect();

            //shuffle array of students
            let mut rng_gen = rng();
            students.shuffle(&mut rng_gen);

            // select group members
            //let grp_members = &self.state.students[0..nbr_of_members];
            //let grp_members = grp_members.iter().map(|s| s).collect();
            //let new_group = Group::from(label, topic, grp_members);
            //groups.push(new_group);
            let grp_members = students
                .iter()
                .take(nbr_of_members)
                .map(|student| {
                    assigned_student_ids.insert(student.get_id());
                    student
                })
                .cloned()
                .collect();

            let mut new_group = Group::from(label, topic.to_owned(), grp_members);
            new_group.set_id(current_group_id as u32);
            println!("{:?}", new_group);
            new_groups.push(new_group);
        }
        self.state.groups.append(&mut new_groups.clone());
    }
    
    pub fn collect_gen_data<T: GenDataId<u32> + Debug + Collect>(elements: &mut Vec<T>) {
        let mut new_element = T::collect();
        new_element.set_id((elements.len() + 1) as u32);
        println!("{:?}", new_element);
        elements.push(new_element);
    }
    pub fn should_break0() -> bool {
        let proceed = DataCollection::input("Do you want to add another topic? (yes/no)[no]");
        !proceed.to_lowercase().eq("yes")   
    }

    pub fn should_break1() -> bool {
        let proceed = DataCollection::input("Do you want to enter another student? (yes/no)[yes]");
        proceed.to_lowercase().eq("no")
    }

    pub fn label_gen(labelling: Labelling, groups_len: usize) -> String {
        match labelling {
            Labelling::Numeric => Self::num_label_gen(groups_len),//state.groups.len().to_string(),//Self::num_label_gen(&self),
            Labelling::Alphabetic => todo!(),
            Labelling::AlphaNumeric => todo!(),
        }
    }
    pub fn num_label_gen(groups_len: usize) -> String {
        (groups_len + 1).to_string()
        //(self.state.groups.len() + 1).to_string()
    }
}

pub struct Helper;

impl Helper {
    pub fn now_in_secs() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH}; 

        let now = SystemTime::now();
        match now.duration_since(UNIX_EPOCH) {
            Ok(value) => value.as_secs(),
            Err(_) => {
                panic!("Time went backwards")
            },
        } 
    }    
}
