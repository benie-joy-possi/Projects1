#[derive(Debug, Clone)]
pub enum Difficulty {
    Hard,
    Medium,
    Easy,
}
impl From<&str> for Difficulty{
    fn from (value: &str) -> Self{
        match value.to_lowercase().as_str() {
            "hard" => Difficulty::Hard,
            "medium" => Difficulty::Medium,
            _ => Difficulty::Easy,
            
        }
    }
}
impl<'a> Into<&str> for Difficulty {
    fn into(self) -> &'static str{
        match self {
            Difficulty::Hard => "hard",
            Difficulty::Medium => "medium",
            Difficulty::Easy => "easy",
        }
    }
}