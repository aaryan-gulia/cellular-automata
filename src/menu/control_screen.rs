use bevy::prelude::*;
use bevy::ui::PositionType::Absolute;
use bevy::{app::MainScheduleOrder, ecs::schedule::*, prelude::*};
use crate::automata::SimState;


pub fn spawn_control_screen(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>){
    let control_screen: Entity = commands.spawn(NodeBundle{
        style:Style{
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::RED.into(),
        ..default()
    })
        .with_children(|parent: &mut ChildBuilder|{
            parent.spawn(ButtonBundle{
                style: Style{
                    position_type: Absolute,
                    align_items:AlignItems::Center,
                    top: Val::Percent(45.0),
                    right: Val::Percent(40.0),
                    width: Val::Percent(20.0),
                    height: Val::Percent(10.0),
                    ..default()
                },
                background_color: Color::rgb(0.15,0.15,0.15).into(),
                ..default()
            }).with_children(
                |parent:&mut ChildBuilder|{
                    parent.spawn(TextBundle{
                        text:Text{
                            sections:vec![TextSection::new("Pause",
                            TextStyle{font:asset_server.load("fonts/Blox2.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE.into(),},
                            )],
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(40.),
                            left: Val::Percent(30.),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    });
                }
            );
        })
        .id();
}

pub fn despawn_control_screen(){

}

pub fn update_control_screen(){

}

pub fn interact_with_pause(
    mut button_query: Query<(&Interaction,&mut BackgroundColor),(Changed<Interaction>)>,
    mut sim_state: ResMut<SimState>
){
    if let Ok((interaction, mut background)) = button_query.get_single_mut(){
        match *interaction {
            Interaction::Hovered=>{*background = Color::rgb(0.25,0.25,0.25).into()},
            Interaction::Pressed=>{
                *background = Color::rgb(0.35,0.35,0.35).into();
                sim_state.is_running = !sim_state.is_running;
            },
            Interaction::None=>{*background = Color::rgb(0.15,0.15,0.15).into()}
        }
    }
}