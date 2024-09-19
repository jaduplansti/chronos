use std::fs::write;

#[derive(Clone)]

struct Task {
    name : String,
    author : String,
    details : String,
    start_date : String,
    run_date : String,
}

struct TaskList {
    list : Vec<Task>,
}


impl TaskList {
    fn new() -> Self {
        return TaskList {list : Vec::new()};
    }

    fn find_by_name(self : &mut Self, name : String) -> Option<Task> {
        for task in &self.list {
            if task.name == name {
                return Some(task.clone());
            }
        }
        return None;
    }

    fn find_by_author(self : &mut Self, author : String) -> Option<Task> {
        for task in &self.list {
            if task.author == author {
                return Some(task.clone());
            }
        }
        return None;
    }
    
    fn find_by_start_date(self : &mut Self, start_date : String) -> Option<Task> {
        for task in &self.list {
            if task.start_date == start_date {
                return Some(task.clone());
            }
        }
        return None;
    }

    fn find_by_run_date(self : &mut Self, run_date : String) -> Option<Task> {
        for task in &self.list {
            if task.run_date == run_date {
                return Some(task.clone());
            }
        }
        return None;
    }
    
    fn tasks_to_string(self : &mut Self) {
        let mut string_of_tasks = String::new();  
        for task in &self.list {
           todo!()     
        }
    }

    fn save(self : &mut Self, filename : String) {
        let _ = write(filename, "");        
    }

}

fn main() {
    
}
