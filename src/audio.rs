use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource, AudioChannel};

pub struct GameAudioPlugin;

pub struct AudioState{
    bgm_handle: Handle<AudioSource>,
    intro_handle: Handle<AudioSource>,

    bgm_channel: AudioChannel,
    sfx_channel: AudioChannel,
    volume: f32,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App){
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_startup_system(start_bgm_music);
    }
}

fn start_bgm_music(audio: Res<Audio>, audio_state: Res<AudioState> ){
    audio.play_looped_in_channel(
        audio_state.bgm_handle.clone(),
        &audio_state.bgm_channel
    );
}

fn load_audio(mut commands: Commands, audio: Res<Audio>, assets: Res<AssetServer>){
    let bgm_handle = assets.load("BGMusic.mp3");
    let intro_handle = assets.load("introMusic.mp3");

    let bgm_channel = AudioChannel::new("bgm".to_string());
    let sfx_channel = AudioChannel::new("sfx".to_string());
    let volume = 0.5;

    audio.set_volume_in_channel(volume, &bgm_channel);
    audio.set_volume_in_channel(volume, &sfx_channel);


    commands.insert_resource(AudioState{
        bgm_handle: bgm_handle,
        intro_handle: intro_handle,
        bgm_channel,
        sfx_channel,
        volume,
    });

}


