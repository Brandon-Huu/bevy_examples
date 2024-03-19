use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::SplashScreen)
                .continue_to_state(GameState::MainMenu)
                .on_failure_continue_to_state(GameState::ErrorScreen)
                .load_collection::<MainMenuAssets>()
        )
        .add_systems(Startup, setup_splash_screen)
        .run();
}

fn setup_splash_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands.spawn(Camera2dBundle::default());

    commands.spawn((TextBundle::from_section(
                "My Game Name",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style { 
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            })
            ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("logos/robot.png"),
        transform: Transform::default()
            .with_scale(Vec3 { x: 0.25, y:0.25, z:0.25 }),
        ..default()
    });
}

#[derive(AssetCollection, Resource)]
struct MainMenuAssets {
}

#[derive(Clone, Eq, PartialEq, Hash, States, Default, Debug)]
enum GameState {
    #[default]
    SplashScreen,
    ErrorScreen,
    MainMenu
}
