//! Shared bounded no-window Bevy render-device initialization; no game data.
use crate::alpha_gpu::GpuResult;
use bevy::{
    app::PluginsState,
    prelude::*,
    render::{
        RenderApp,
        pipelined_rendering::PipelinedRenderingPlugin,
        renderer::{RenderAdapterInfo, RenderDevice, RenderQueue},
    },
    window::{ExitCondition, WindowPlugin},
    winit::WinitPlugin,
};
use std::time::{Duration, Instant};
pub fn setup() -> GpuResult<(App, RenderDevice, RenderQueue)> {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..Default::default()
            })
            .disable::<WinitPlugin>()
            .disable::<PipelinedRenderingPlugin>(),
    );
    let start = Instant::now();
    while app.plugins_state() != PluginsState::Ready {
        if start.elapsed() > Duration::from_secs(30) {
            return Err("GPU plugin initialization timeout".into());
        }
        bevy::tasks::tick_global_task_pools_on_main_thread();
        std::thread::sleep(Duration::from_millis(1));
    }
    app.finish();
    app.cleanup();
    let render = app.sub_app(RenderApp).world();
    let info = render.resource::<RenderAdapterInfo>();
    println!("GPU adapter: {} / {:?}", info.name, info.backend);
    let device = render.resource::<RenderDevice>().clone();
    let queue = render.resource::<RenderQueue>().clone();
    Ok((app, device, queue))
}
