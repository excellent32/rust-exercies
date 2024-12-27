use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, Write};

pub struct Record{
    pub id: i32,
    pub content: String
}

pub struct Database{
    pub file: File,
}

impl From<&str> for Record {
    fn from(line: &str) -> Record{
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() == 1 {
            return Record{
                id: 0,
                content: "".to_string(),
            }
        }

        let content = fields[1..].join(",");
        Record{
            id: fields[0].parse::<i32>().unwrap(),
            content,
        }
    }
}

impl Database{
    pub fn open(filename: &str) -> Database {
        let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(filename)
        .unwrap();

        Database{file}
    }

    pub fn add_record(&mut self, record: &Record){
        let line = format!("{},{}\n", record.id, record.content);
        writeln!(self.file, "{}", line).unwrap();
        println!("Item added: {}", record.content)
    }

    pub fn read_records(&mut self) -> Vec<Record>{
        let reader = BufReader::new(&self.file);
        reader.lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line|  Record::from(line.as_str()))
        .collect()
    }

    pub fn remove_record(&mut self, id: i32) {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();
        let line = lines.find(|(_, line)| {
            let record = Record::from(line.as_ref().unwrap().as_str());
            record.id == id
        });

        match line {
            Some((i, _)) => {
                let contents = fs::read_to_string(".todorc").unwrap();
                let new_contents =  contents.lines()
                .enumerate()
                .filter(|(j,_)| *j !=i)
                .map(|(_,line)| line)
                .collect::<Vec<_>>()
                .join("\n");

                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_contents.as_bytes()).unwrap();
                self.file.set_len(new_contents.len() as u64).unwrap();

                println!(" Item removed!\n");
            }
            None => {
                println!("No such record: {}", id);
            }
        }
    }
}