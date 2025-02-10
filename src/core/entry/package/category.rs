use std::{fmt::Display, str::FromStr};

use toml_edit::{Formatted, Item, Value};

/// # AppCategory
/// "Business" | "DeveloperTool" | "Education" | "Entertainment" | "Finance" | "Game" |
/// "ActionGame" | "AdventureGame" | "ArcadeGame" | "BoardGame" | "CardGame" | "CasinoGame" |
/// "DiceGame" | "EducationalGame" | "FamilyGame" | "KidsGame" | "MusicGame" | "PuzzleGame" |
/// "RacingGame" | "RolePlayingGame" | "SimulationGame" | "SportsGame" | "StrategyGame" |
/// "TriviaGame" | "WordGame" | "GraphicsAndDesign" | "HealthcareAndFitness" | "Lifestyle" |
/// "Medical" | "Music" | "News" | "Photography" | "Productivity" | "Reference" |
/// "SocialNetworking" | "Sports" | "Travel" | "Utility" | "Video" | "Weather"
///
/// The possible app categories. Corresponds to LSApplicationCategoryType on macOS and the
/// GNOME desktop categories on Debian.
#[derive(Debug, Clone, Copy)]
pub enum AppCategory {
    Business,
    DeveloperTool,
    Education,
    Entertainment,
    Finance,
    Game,
    ActionGame,
    AdventureGame,
    ArcadeGame,
    BoardGame,
    CardGame,
    CasinoGame,
    DiceGame,
    EducationalGame,
    FamilyGame,
    KidsGame,
    MusicGame,
    PuzzleGame,
    RacingGame,
    RolePlayingGame,
    SimulationGame,
    SportsGame,
    StrategyGame,
    TriviaGame,
    WordGame,
    GraphicsAndDesign,
    HealthcareAndFitness,
    Lifestyle,
    Medical,
    Music,
    News,
    Photography,
    Productivity,
    Reference,
    SocialNetworking,
    Sports,
    Travel,
    Utility,
    Video,
    Weather,
}

impl Display for AppCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(self).to_string().as_str())
    }
}

impl From<&AppCategory> for Value {
    fn from(app_category: &AppCategory) -> Self {
        Value::String(Formatted::new(
            match app_category {
                AppCategory::Business => "Business",
                AppCategory::DeveloperTool => "DeveloperTool",
                AppCategory::Education => "Education",
                AppCategory::Entertainment => "Entertainment",
                AppCategory::Finance => "Finance",
                AppCategory::Game => "Game",
                AppCategory::ActionGame => "ActionGame",
                AppCategory::AdventureGame => "AdventureGame",
                AppCategory::ArcadeGame => "ArcadeGame",
                AppCategory::BoardGame => "BoardGame",
                AppCategory::CardGame => "CardGame",
                AppCategory::CasinoGame => "CasinoGame",
                AppCategory::DiceGame => "DiceGame",
                AppCategory::EducationalGame => "EducationalGame",
                AppCategory::FamilyGame => "FamilyGame",
                AppCategory::KidsGame => "KidsGame",
                AppCategory::MusicGame => "MusicGame",
                AppCategory::PuzzleGame => "PuzzleGame",
                AppCategory::RacingGame => "RacingGame",
                AppCategory::RolePlayingGame => "RolePlayingGame",
                AppCategory::SimulationGame => "SimulationGame",
                AppCategory::SportsGame => "SportsGame",
                AppCategory::StrategyGame => "StrategyGame",
                AppCategory::TriviaGame => "TriviaGame",
                AppCategory::WordGame => "WordGame",
                AppCategory::GraphicsAndDesign => "GraphicsAndDesign",
                AppCategory::HealthcareAndFitness => "HealthcareAndFitness",
                AppCategory::Lifestyle => "Lifestyle",
                AppCategory::Medical => "Medical",
                AppCategory::Music => "Music",
                AppCategory::News => "News",
                AppCategory::Photography => "Photography",
                AppCategory::Productivity => "Productivity",
                AppCategory::Reference => "Reference",
                AppCategory::SocialNetworking => "SocialNetworking",
                AppCategory::Sports => "Sports",
                AppCategory::Travel => "Travel",
                AppCategory::Utility => "Utility",
                AppCategory::Video => "Video",
                AppCategory::Weather => "Weather",
            }
            .to_string(),
        ))
    }
}

impl TryFrom<&Item> for AppCategory {
    type Error = gen_utils::error::Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        match value.as_str() {
            Some(v) => v.parse(),
            None => Err(gen_utils::error::ConvertError::FromTo {
                from: "toml::Item".to_string(),
                to: "AppCategory".to_string(),
            }
            .into()),
        }
    }
}

impl FromStr for AppCategory {
    type Err = gen_utils::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Business" => Ok(Self::Business),
            "DeveloperTool" => Ok(Self::DeveloperTool),
            "Education" => Ok(Self::Education),
            "Entertainment" => Ok(Self::Entertainment),
            "Finance" => Ok(Self::Finance),
            "Game" => Ok(Self::Game),
            "ActionGame" => Ok(Self::ActionGame),
            "AdventureGame" => Ok(Self::AdventureGame),
            "ArcadeGame" => Ok(Self::ArcadeGame),
            "BoardGame" => Ok(Self::BoardGame),
            "CardGame" => Ok(Self::CardGame),
            "CasinoGame" => Ok(Self::CasinoGame),
            "DiceGame" => Ok(Self::DiceGame),
            "EducationalGame" => Ok(Self::EducationalGame),
            "FamilyGame" => Ok(Self::FamilyGame),
            "KidsGame" => Ok(Self::KidsGame),
            "MusicGame" => Ok(Self::MusicGame),
            "PuzzleGame" => Ok(Self::PuzzleGame),
            "RacingGame" => Ok(Self::RacingGame),
            "RolePlayingGame" => Ok(Self::RolePlayingGame),
            "SimulationGame" => Ok(Self::SimulationGame),
            "SportsGame" => Ok(Self::SportsGame),
            "StrategyGame" => Ok(Self::StrategyGame),
            "TriviaGame" => Ok(Self::TriviaGame),
            "WordGame" => Ok(Self::WordGame),
            "GraphicsAndDesign" => Ok(Self::GraphicsAndDesign),
            "HealthcareAndFitness" => Ok(Self::HealthcareAndFitness),
            "Lifestyle" => Ok(Self::Lifestyle),
            "Medical" => Ok(Self::Medical),
            "Music" => Ok(Self::Music),
            "News" => Ok(Self::News),
            "Photography" => Ok(Self::Photography),
            "Productivity" => Ok(Self::Productivity),
            "Reference" => Ok(Self::Reference),
            "SocialNetworking" => Ok(Self::SocialNetworking),
            "Sports" => Ok(Self::Sports),
            "Travel" => Ok(Self::Travel),
            "Utility" => Ok(Self::Utility),
            "Video" => Ok(Self::Video),
            "Weather" => Ok(Self::Weather),
            _ => Err(gen_utils::error::ConvertError::FromTo {
                from: "&str".to_string(),
                to: format!("AppCategory, Invalid: {}", s),
            }
            .into()),
        }
    }
}
