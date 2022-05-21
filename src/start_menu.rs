use bevy::prelude::*;
use bevy::ui::FocusPolicy;

pub struct MainMenuPlugin;

//Check 32 seconds for making a UI camera seperate from Main

struct UiAssets {
    font: Handle<Font>,
    button: Handle<Image>,
    button_pressed: Handle<Image>
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {}
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = UiAssets {
        font: assets.load("QuattrocentoSans-Bold.ttf"),
        button: assets.load("start_1.png"),
        button_pressed: assets.load("start_clicked_1.png")
    };

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(15.0), Val::Percent(10.0)),
            margin: Rect::all(Val::Auto),
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }).with_children(|parent: &mut ChildBuilder| {
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

