use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState{ //게임 상태
    #[default] 
    MainMenu, //초기화 값은 메인메뉴라는 명시
    Game,
    GameOver
}