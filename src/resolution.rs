
use bevy::prelude::*;  // Bevy의 기본 기능을 사용하기 위해 prelude를 가져옵니다.

/// 해상도 관련 로직을 처리하는 ResolutionPlugin 플러그인을 정의합니다.
pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    // 플러그인이 애플리케이션에 추가될 때 실행되는 빌드 함수입니다.
    fn build(&self, app: &mut App) {
        // PreStartup 단계에서 `setup_resolution` 시스템을 실행합니다.
        // PreStartup은 게임 내의 다른 Startup 함수들이 실행되기 전에 먼저 실행됩니다.
        app.add_systems(PreStartup, setup_resolution);
    }
}

// 해상도 정보를 저장할 `Resolution` 리소스를 정의합니다.
#[derive(Resource)]
pub struct Resolution {
    // 화면의 픽셀 크기를 2D 벡터 (width, height) 형태로 저장합니다.
    pub screen_dimensions: Vec2,
    // 스프라이트 픽셀과 화면상의 픽셀 비율을 저장합니다.
    pub pixel_ratio: f32,
}

// 해상도를 설정하는 함수입니다.
fn setup_resolution(
    mut commands: Commands,         // 리소스를 삽입하거나 엔티티를 생성할 수 있는 명령 객체입니다.
    window_query: Query<&Window>    // 윈도우 정보를 쿼리합니다.
) {
    // 윈도우 정보를 가져옵니다.
    let window = window_query.single();

    // 화면의 크기와 픽셀 비율을 포함하는 `Resolution` 리소스를 삽입합니다.
    commands.insert_resource(Resolution {
        screen_dimensions: Vec2::new(window.width(), window.height()),  // 윈도우의 너비와 높이를 가져와 화면 크기를 설정합니다.
        pixel_ratio: 2.0,  // 스프라이트의 픽셀과 화면 픽셀 간의 비율을 2.0으로 설정합니다.
    });
}