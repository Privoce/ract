use std::fmt::Display;

use toml_edit::{Formatted, Value};

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
