use bevy::prelude::Component;

#[derive(Component)]
pub enum ClassType {
    Hunter = 0,
    Cardinal = 1,
    Forager = 2,
    Warrior = 3,
    Thief = 4,
    Alchemist = 5,
    Merchant = 6,
    Bard = 7,
    GuildMaster = 8,
}

impl ClassType {
    pub fn name(self) -> String {
        match self {
            ClassType::Hunter => "Hunter".to_string(),
            ClassType::Cardinal => "Cardinal".to_string(),
            ClassType::Forager => "Forager".to_string(),
            ClassType::Warrior => "Warrior".to_string(),
            ClassType::Thief => "Thief".to_string(),
            ClassType::Alchemist => "Alchemist".to_string(),
            ClassType::Merchant => "Merchant".to_string(),
            ClassType::Bard => "Bard".to_string(),
            ClassType::GuildMaster => "GuildMaster".to_string(),
        }
    }
    pub fn index_of(index: u8) -> Self {
        match index {
            0 => ClassType::Hunter,
            1 => ClassType::Cardinal,
            2 => ClassType::Forager,
            3 => ClassType::Warrior,
            4 => ClassType::Thief,
            5 => ClassType::Alchemist,
            6 => ClassType::Merchant,
            7 => ClassType::Bard,
            8 => ClassType::GuildMaster,
            _ => panic!("Invalid index"),
        }
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_name() {
//         assert_eq!(ClassType::Hunter.name(), "Hunter".to_string());
//     }
// }