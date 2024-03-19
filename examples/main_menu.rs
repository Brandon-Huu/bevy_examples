use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

fn main() {
    App::new()
        .insert_resource(Time::default())
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::SplashScreen)
                .continue_to_state(GameState::MainMenu)
                .on_failure_continue_to_state(GameState::ErrorScreen)
                .load_collection::<MainMenuAssets>()
        )
        .add_systems(OnEnter(GameState::SplashScreen), setup_splash_screen)
        .add_systems(OnExit(GameState::SplashScreen), despawn_splash_screen )
        .add_system(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(Update, timeout.run_if(in_state(GameState::SplashScreen)))
        .run();
}

fn setup_main_menu(
    mut commands: Commands,
) {
}

fn timeout(time: Res<Time>){
    info!("{:?}", time.elapsed_seconds_f64());
    if time.elapsed_seconds_f64() < 10 { return; }
    panic!("");
}

#[derive(Component)]
struct SplashScreenElement {}

fn despawn_splash_screen(
    mut commands: Commands,
    query: Query<(Entity, &SplashScreenElement)>
){
    for element in &mut query {
        commands.entity(element).despawn_recursive());
    }
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