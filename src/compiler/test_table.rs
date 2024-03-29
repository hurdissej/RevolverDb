use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::mem;
use std::fs::OpenOptions;
use std::fs;

pub struct Table {
    pub persistence: File,
    pub max_page_size: i32,
    pub number_of_rows: i32,
    pub pages: Vec<Page>
}

pub struct Page {
    pub full: bool,
    pub rows: Vec<Row>
}

#[derive(Clone)]
pub struct Row {
    pub id: i32,
    pub username: Vec<u8>,
    pub email: Vec<u8>
}

//TODO make this Generic table - and implement storage for tables
impl Row {
    pub fn new(id: i32, username: &str, email: &str) -> Row{
        if username.len() > 32{
            panic!("Username too long");
        }
        if email.len() > 255{
            panic!("Email too long");
        }
        
        return Row{
            id: id,
            username: to_bytes(username, 32),
            email: to_bytes(email, 255),
        }
    }    
}

impl Table {
    pub fn new() -> Table{
        return match OpenOptions::new()
            .read(true)
            .append(true)
            .create(true).open("db.db") {
            Ok(table) => build_table(table),
            Err(err) =>  panic!(err)
        };
    }
    //TODO handle errors
    pub fn persist(& mut self, row_to_insert: &Row)
    {    
        self.persistence.write(&row_to_insert.id.to_le_bytes()).unwrap();
        self.persistence.write(row_to_insert.username.as_slice()).unwrap();
        self.persistence.write(row_to_insert.email.as_slice()).unwrap();
    }

    pub fn insert(& mut self, row_to_insert: Row){
        let index_for_page = self.pages.len() - 1;
        if !self.pages[index_for_page].full {
            &self.persist(&row_to_insert);
            self.pages[index_for_page].rows.push(row_to_insert);
        } else {
            let mut rows: Vec<Row> = Vec::new();
            rows.push(row_to_insert);
            let page = Page {
                full: false,
                rows: rows
            };

            //TODO Check if it is full now
            &self.pages.push(page);
        }
    }
}

fn build_table(db_file: File) -> Table {
    let num = fs::metadata("db.db").unwrap().len() / 291;
    let mut offset = 0;
    let mut rows: Vec<Row> = Vec::new();        
    println!("{}", num);

    let mut buffer: Vec<u8> = Vec::new();
    let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("db.db").unwrap(); 
    
    file.read_to_end(&mut buffer).unwrap();

    for _row in 0..num {
        let row = read_row(&buffer[offset..offset+291]);
        rows.push(row);
        offset += 291;
    }

    let mut pages: Vec<Page> = Vec::new();
    
    let page = Page {
        full: false,
        rows: rows
    };

    pages.push(page);
    Table {
        persistence: db_file,
        max_page_size: 100, 
        number_of_rows: 0,
        pages: pages
    }        
}

fn read_row(row: &[u8]) -> Row {
    let mut a: [u8; 4] = Default::default();
    a.copy_from_slice(&row[0..4]);
    let id = unsafe{ mem::transmute::<[u8; 4], i32>(a) };
    let username = &row[4..35].to_vec();
    let email = &row[36..290].to_vec();

    Row {
        id: id,
        username: username.clone(),
        email: email.clone()
    }
}

fn to_bytes(input: &str, size: usize) -> Vec<u8> {
        let mut a:Vec<u8> = std::iter::repeat(0).take(size).collect::<Vec<_>>();
        let bytes_to_copy = input.as_bytes();
        
        if bytes_to_copy.len() > size {
            panic!("Too long");
        }

        a[..bytes_to_copy.len()].clone_from_slice(bytes_to_copy);
        a        
    }