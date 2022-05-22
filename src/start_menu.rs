use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crate::{GameState, LevelNumber};

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct ButtonActive(bool);

struct UiAssets {
    font: Handle<Font>,
    button: Handle<Image>,
    button_pressed: Handle<Image>
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_menu)
            .add_system_set(SystemSet::on_exit(GameState::Splash).with_system(despawn_menu))
            .add_system(handle_start_button);
    }
}

fn despawn_menu(mut commands: Commands, button_query: Query<Entity, With<Button>>) {
    for ent in button_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn handle_start_button(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Children, &mut ButtonActive, &Interaction),
        Changed<Interaction>,
    >,
    mut image_query: Query<&mut UiImage>,
    ui_assets: Res<UiAssets>,
    mut game_state: ResMut<State<GameState>>,
    mut level_state: ResMut<State<LevelNumber>>,
    //ascii: Res<AsciiSheet>,
) {
    for (children, mut active, interaction) in interaction_query.iter_mut() {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();

        match interaction {
            Interaction::Clicked => {
                if active.0 {
                    image.0 = ui_assets.button_pressed.clone();
                    game_state.set(GameState::Game).unwrap();
                    level_state.set(LevelNumber::One).unwrap();
                    //Here change gamestate
                    //create_fadeout(&mut commands, Some(GameState::Splash), &ascii);
                    active.0 = false;
                }
            }
            Interaction::Hovered | Interaction::None => {
                image.0 = ui_assets.button.clone();
            }
        }
    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = UiAssets {
        font: assets.load("Ubuntu-Bold.ttf"),
        button: assets.load("start_1.png"),
        button_pressed: assets.load("start_clicked_1.png")
    };

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(50.0), Val::Percent(40.0)),
            margin: Rect::all(Val::Auto),
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
        .insert(ButtonActive(true))
        .with_children(|parent: &mut ChildBuilder| {
        parent.spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            image: ui_assets.button.clone().into(),
            ..Default::default()
        })
            .insert(FocusPolicy::Pass).with_children(|parent: &mut ChildBuilder| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Start Game",
                    TextStyle {
                        font: ui_assets.font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default()),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });
        });
    });
    commands.insert_resource(ui_assets);

}

