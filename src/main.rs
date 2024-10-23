
use bevy::prelude::*;   // 게임 루프 및 주요 구조체들을 사용하기 위해 Bevy의 기본 프리루드를 가져옵니다.

// 이 프로젝트에서 사용할 모듈들을 선언합니다.
pub mod game;       // 게임 로직을 담당하는 모듈입니다.
pub mod alien;      // 플레이어에게 다가오는 외계인을 담당하는 모듈입니다.
pub mod resolution; // 해상도 설정을 담당하는 모듈입니다.
pub mod player;     // 플레이어를 담당하는 모듈입니다.
pub mod projectile; // 투사체를 담당하는 모듈입니다.

fn main() {
    App::new()  // 새로운 Bevy 애플리케이션을 생성합니다.
        .add_plugins(
            (
                // 게임에 추가할 플러그인들의 목록입니다.
                DefaultPlugins  // Bevy에서 제공하는 기본 플러그인 세트를 추가합니다.
                .set(WindowPlugin {                     // 윈도우 설정을 사용자 지정합니다.
                    primary_window: Some(Window {                   // 주 창의 속성을 설정합니다.
                        title: String::from("Space Invaders"),                          // 창 제목을 'Space Invaders'로 설정합니다.
                        position: WindowPosition::Centered(MonitorSelection::Primary),  // 창을 모니터 중앙에 위치시킵니다.
                        resolution: Vec2::new(512., 512.).into(),                  // 창의 해상도를 512x512로 설정합니다.
                        ..Default::default()                                            // 나머지 설정은 기본값을 사용합니다.
                    }),
                    ..Default::default()  // WindowPlugin의 다른 설정도 기본값을 사용합니다.
                })
                .set(ImagePlugin::default_nearest()),   // 텍스처가 선명하게 보이도록 이미지 플러그인을 설정합니다.

                game::GamePlugin,  // 우리의 게임 로직을 처리하는 GamePlugin을 추가합니다.
            ),
        )
        .run();  // 애플리케이션을 실행합니다.
}
