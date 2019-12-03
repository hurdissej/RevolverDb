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

fn to_bytes(input: &str, size: usize) -> Vec<u8> {
        let mut a:Vec<u8> = std::iter::repeat(0).take(size).collect::<Vec<_>>();
        let bytes_to_copy = input.as_bytes();

        if bytes_to_copy.len() > size {
            panic!("Too long");
        }

        a[..bytes_to_copy.len()].clone_from_slice(bytes_to_copy);
        a        
    }