pub struct Table {
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
        let mut pages: Vec<Page> = Vec::new(); //TODO Make into a constructor
        let rows: Vec<Row> = Vec::new();        
        let page = Page {
            full: false,
            rows: rows
        };
        pages.push(page);
        
        Table {
            max_page_size: 100, 
            number_of_rows: 0,
            pages: pages
        }
        
    }

    pub fn insert(& mut self, row_to_insert: Row){
        let index_for_page = self.pages.len() - 1;
        if !self.pages[index_for_page].full {
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

fn to_bytes(input: &str, size: usize) -> Vec<u8> {
        let mut a:Vec<u8> = std::iter::repeat(0).take(size).collect::<Vec<_>>();
        let bytes_to_copy = input.as_bytes();

        if bytes_to_copy.len() > size {
            panic!("Too long");
        }

        a[..bytes_to_copy.len()].clone_from_slice(bytes_to_copy);
        a        
    }