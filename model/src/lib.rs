use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShoppingListItem {
    pub title: String,
    pub posted_by: String,
    pub uuid: String,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
