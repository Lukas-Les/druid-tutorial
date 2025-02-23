use druid::widget::{Button, Flex, Scroll, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use druid::commands::SHOW_OPEN_PANEL;
use druid::{FileDialogOptions, FileSpec, Target, Selector, Command};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Data, Lens)]
struct AppState {
    file_content: String,
}

const UPDATE_CONTENT: Selector<String> = Selector::new("update-content");

fn build_ui() -> impl Widget<AppState> {
    let button = Button::new("Select a file")
        .on_click(|ctx, _data: &mut AppState, _env| {
            let options = FileDialogOptions::new()
                .allowed_types(vec![FileSpec::new("Text files", &["txt", "json", "html, xml"])])
                .default_type(FileSpec::new("Text file", &["txt"]))
                .name_label("Select a file")
                .title("Choose a file to open")
                .button_text("Search");
            ctx.submit_command(SHOW_OPEN_PANEL.with(options).to(Target::Auto));
        });

    let file_content = TextBox::multiline()
        .with_placeholder("File content will be displayed here")
        .lens(AppState::file_content);

    let layout = Flex::column()
        .with_child(button)
        .with_spacer(8.0)
        .with_child(Scroll::new(file_content).vertical());

    layout
}

fn main() -> Result<(), druid::PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("simple-druid-app").with_placeholder("Simple Druid App"))
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .delegate(AppDelegate)
        .launch(AppState {
            file_content: "".to_string(),
        })
        .expect("Failed to launch application");

    Ok(())
}

struct AppDelegate;

impl druid::AppDelegate<AppState> for AppDelegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &Env,
    ) -> druid::Handled {
        if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {
            if let Some(path) = file_info.path().to_str() {
                if let Ok(mut file) = File::open(path) {
                    let mut buffer = String::new();
                    if file.read_to_string(&mut buffer).is_ok() {
                        ctx.submit_command(Command::new(UPDATE_CONTENT, buffer, Target::Auto));
                    }
                }
            }
            return druid::Handled::Yes;
        }

        if let Some(content) = cmd.get(UPDATE_CONTENT) {
            data.file_content = content.clone();
            return druid::Handled::Yes;
        }

        druid::Handled::No
    }
}
