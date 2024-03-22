#![allow(unused)]
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource::<Time>(Time::default())
        .add_plugins((DefaultPlugins, /* SplashScreenPlugin, */ MainMenuPlugin))
        .init_state::<GameState>()
        .run();
}

#[derive(Clone, Eq, PartialEq, Hash, States, Default, Debug)]
enum GameState {
    #[default]
    //SplashScreen,
    //ErrorScreen,
    MainMenu,
}

pub struct MainMenuPlugin;

#[derive(Component)]
struct MainMenuElement {}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer> ) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            MainMenuElement {},
            ButtonBundle {
                style: Style {
                    height: Val::Px(108.0),
                    width: Val::Px(192.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                border_color: BorderColor(Color::BLACK),
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn((
                MainMenuElement {},
                TextBundle::from_section(
                    "Start Game",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
            ));
        });
}
