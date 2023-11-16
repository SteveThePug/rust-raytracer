struct ImguiDemo {
    canvas: framework::ShaderCanvas,
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: Renderer,
    last_cursor: Option<MouseCursor>,
}
