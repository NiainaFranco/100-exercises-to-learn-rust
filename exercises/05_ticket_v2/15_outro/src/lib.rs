// TODO: you have something to do in each of the modules in this crate!
mod description;
mod status;
mod title;

// A common pattern in Rust is to split code into multiple (private) modules
// and then re-export the public parts of those modules at the root of the crate.
//
// This hides the internal structure of the crate from your users, while still
// allowing you to organize your code however you like.
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// We no longer need to make the fields private!
// Since each field encapsulates its own validation logic, there is no risk of
// a user of `Ticket` modifying the fields in a way that would break the
// invariants of the struct.
//
// Careful though: if you had any invariants that spanned multiple fields, you
// would need to ensure that those invariants are still maintained and go back
// to making the fields private.
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

pub trait OnlyString {
    fn only_string(&self)->String;
}

impl OnlyString for String {
    fn only_string(&self)->String {
        self.clone()
    }
}

impl OnlyString for &str {
    fn only_string(&self)->String {
        self.to_string()
    }
}

pub struct TicketBuilder {
    title: Option<TicketTitle>,
    description: Option<TicketDescription>,
    status: Option<Status>,
    error: Option<String>
}

impl TicketBuilder {
    pub fn new()->Self{
        TicketBuilder { title: None, description: None, status: None, error: None }
    }

    pub fn description<T: OnlyString>(&mut self, value: T)->&Self{
        let clean_value= TicketDescription::try_from(value.only_string());
        if clean_value.is_err() {
            self.error = Some(clean_value.unwrap_err());
            return self;
        }
        self.description = Some(clean_value.unwrap());
        self
    }

    pub fn title <T: OnlyString>(&mut self, value: T)->&Self{
        let clean_value= TicketTitle::try_from(value.only_string());
        if clean_value.is_err() {
            self.error = Some(clean_value.unwrap_err().to_string());
            return self;
        }
        self.title = Some(clean_value.unwrap());
        self
    }

    pub fn status <T: OnlyString>(&mut self, value: T)->&Self{
        let clean_value= Status::try_from(value.only_string());
        if clean_value.is_err() {
            self.error = Some(clean_value.unwrap_err().to_string());
            return self;
        }
        self.status = Some(clean_value.unwrap());
        self
    }
    
    fn check_error(&self){
       if self.error.is_some() {
        panic!();
       }
    }
    
    fn check_values(&self){
        if self.description.is_none() {
            panic!("description must be set");
        }
        if self.title.is_none() {
            panic!("title must be set");
        }
        if self.status.is_none() {
            panic!("status must be set");
        }
    }
    
    pub fn create (&self)-> Ticket {
        self.check_error();
        self.check_values();
        Ticket{description: self.description.clone().unwrap(), status: self.status.clone().unwrap(), title: self.title.clone().unwrap()}
    }
    
}
