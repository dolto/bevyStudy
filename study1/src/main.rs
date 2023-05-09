use bevy::prelude::*;
use stody1::hello_world::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloWorldPlugin)
    .run();
}

//add_system(함수) 앱이 수행되는 프레임당 한번씩 수행됨
//add_startup_system(함수) 앱이 시작되면 최초 한번만 수행하는 함수