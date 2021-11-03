use druid::{widget::Scroll, AppLauncher, PlatformError, Widget, WindowDesc};

mod consts;
mod data;
mod tools;
mod widgets;

use data::ApplicationState;
use widgets::{grid::CanvasGrid, layout::StackLayout, status_label::StatusLabel};

fn build_ui() -> impl Widget<ApplicationState> {
    let mut ui = StackLayout::new();
    ui.add_child(Scroll::new(CanvasGrid::new()));
    ui.add_child(StatusLabel::new());
    ui
}

fn main() -> Result<(), PlatformError> {
    // https://github.com/linebender/druid/pull/1701/files
    // Follow the above PR for transparent title bar status
    let app = AppLauncher::with_window(WindowDesc::new(build_ui()));
    app.launch(ApplicationState {
        mode: String::new(),
    })?;
    Ok(())
}
