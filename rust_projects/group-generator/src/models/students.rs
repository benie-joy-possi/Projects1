use crate::{
    data_collection::DataCollection,
    traits::{
        collect::Collect,
        gen_data_id::{self, GenDataId},
    },
};

#[derive(Debug, Clone)]
pub struct Student {
    id: u32,
    name: String,
}
impl Student {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl Collect for Student {
    fn collect() -> Self {
        let mut student = Self::new();
        // println!("Enter the topics \n Type 'done' when completed");

        // let prompt = Some("Enter the student's name:".to_string());
        student.name = DataCollection::input("Enter the student's name:");

        student
    }
}
impl GenDataId<u32> for Student {
    fn set_id(&mut self, id: u32) {
        self.id = id
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
