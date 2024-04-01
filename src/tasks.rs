use std::{fmt, fs::{File, OpenOptions}, io::{self, Error, Seek, SeekFrom}, path::PathBuf};
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub text:String, 
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>
}
impl Task {
    pub fn new(text:String) -> Task{
        return Task { text, created_at: Utc::now()}
    }
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

fn collect_tasks(mut file:&File) -> io::Result<Vec<Task>>{
    file.seek(SeekFrom::Start(0))?; // Rewind the file before and after
    let tasks = match serde_json::from_reader(file){
        Ok(tasks) => tasks,
        // when its emthy or so 
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?
    };
     // this point we have properly parsed our collection of task so we need to 
    // reset position with seekfrom of the file for future operations with this file
    file.seek(SeekFrom::Start(0))?; 
    return Ok(tasks)
}

pub fn add_task(route_file:PathBuf, task:Task) -> io::Result<()>{
    // ? its suggar sintax for returning an error if the function doesnt return the expectect type(this case File) 
    // we can avoid this way getting a Result<File> on every operation this way and just go with the add_task io::Err it something went wrong
    // OpenOptions just to mannage more options but similar to File::open
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        // to create in case it does not exist the file 
        .create(true)
        .open(route_file)?;

    //consume the file content into an vector (kind of parse it)
    let mut tasks:Vec<Task> = collect_tasks(&file)?;
    //finally write the file 
    tasks.push(task);
    serde_json::to_writer_pretty(file, &tasks)?;
    return Ok(())
}

pub fn complete_task(route_file:PathBuf, task_position:usize) -> io::Result<()>{
    if task_position == 0 {
        return Err(Error::new(io::ErrorKind::InvalidInput, "0 It Is Not A Valid Task id"))
    };

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(route_file)?;

    let mut tasks = collect_tasks(&file)?;

    if task_position > tasks.len(){
        return Err(
            Error::new(io::ErrorKind::InvalidInput, "Invalid Task ID"))
    };
    tasks.remove(task_position - 1);
    // Write the modified task list back into the file.
    // When we truncate the file by using the file.set_len(0) operation, we ensure that we're writing the bytes in a blank page.
    file.set_len(0)?;
    //We're truncating the file before writing to it because we're performing a removal operation. So the file will be smaller than the original. If we ignored this step, the rewound cursor would stop behind the previously written bytes of the file, resulting in a malformed JSON file
    serde_json::to_writer(file, &tasks)?;
    return Ok(())
}

pub fn get_list(route_file:PathBuf) -> io::Result<()>{
    let file = OpenOptions::new().read(true).open(route_file)?;
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() { println!("Empty List");
    }else{
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    };
    return Ok(())
}

