#![allow(unused)]
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

fn main() {
    App::new()
        .insert_resource::<Time>(Time::default())
        .add_plugins((DefaultPlugins, SplashScreenPlugin))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::SplashScreen)
                .continue_to_state(GameState::MainMenu)
                .on_failure_continue_to_state(GameState::ErrorScreen)
                .load_collection::<MainMenuAssets>()
        )
        .run();
}

#[derive(Component)]
struct SplashScreenElement {}

#[derive(AssetCollection, Resource)]
struct MainMenuAssets {
    #[asset(path="logos/robot.png")]
    robot: Handle<Image>
}

#[derive(Clone, Eq, PartialEq, Hash, States, Default, Debug)]
enum GameState {
    #[default]
    SplashScreen,
    ErrorScreen,
    MainMenu
}

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SplashScreen), setup_splash_screen)
            .add_systems(OnExit(GameState::SplashScreen), despawn_splash_screen)
            .add_systems(Update, timeout.run_if(in_state(GameState::SplashScreen)));

    }
}

fn timeout(time: Res<Time>){
    info!("{:?}", time.elapsed_seconds_f64());
    if time.elapsed_seconds_f64() < 10.0 { return; }
    panic!("");
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
            }),
            SplashScreenElement {} ));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("logos/robot.png"),
        transform: Transform::default()
            .with_scale(Vec3 { x: 0.25, y:0.25, z:0.25 }),
        ..default()
    },
    SplashScreenElement {}));
}

fn despawn_splash_screen(
    mut commands: Commands,
    mut query: Query<(Entity, &SplashScreenElement)>
){
    for (entity, _element) in &mut query {
        commands.entity(entity).despawn_recursive();
    }
}
