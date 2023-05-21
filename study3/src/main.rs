mod bird;
mod score;
mod pipe;
mod menu;
mod steat;
mod events;
mod camera;

use bevy::prelude::*;
use bird::BirdPlugin;
use events::EventPlugin;
use pipe::PipPlugin;
use camera::camera_spawn;
use score::ScorePlugin;
use steat::AppState;

fn main() {
    App::new()
    .add_state::<AppState>()
    .add_startup_system(camera_spawn)
    .add_plugins(DefaultPlugins)
    .add_plugin(EventPlugin)
    .add_plugin(BirdPlugin)
    .add_plugin(PipPlugin)
    .add_plugin(ScorePlugin)
    .run();
}
/***
 * 웹 어셈블러로 빌드하기
 * 
 * 먼저 러스트에 기능추가
 * rustup target install wasm32-unknown-unknown
 * 
 * cargo install wasm-server-runner
 * 
 * ..cargo/config.toml (보통 User에 cargo폴더가 있고, config.toml이 없다면 만들어서 쓴다.)
 * [target.wasm32-unknown-unknown]
 * runner = "wasm-server-runner"
 * 
 * 혹은 환경변수로 지정
 * export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner
 * 
 * 이제 이렇게 써서 로컬 호스트로 wasm으로 수행하는 브라우저용 게임 실행
 * cargo run --target wasm32-unknown-unknown
 * 메모리가 너무 많다고 어쩌꾸 뜨면
 * cargo run --release --target wasm32-unknown-unknown
 * 이렇게 릴리즈 버전으로 수행한다.
 * 
 * 이러면 웹사이트에서 실행하기 위한 모든 필요요소를 제작
 * cargo build --release --target wasm32-unknown-unknown
 * wasm-bindgen --out-dir ./out/ --target web ./target/
 * 
 */