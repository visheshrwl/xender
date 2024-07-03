use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppState {
    file_path: String,
    status: String,
}

fn build_ui() -> impl Widget<AppState> {
    let file_path_input = TextBox::new()
        .with_placeholder("Enter file path")
        .lens(AppState::file_path);

    let status_label = Label::dynamic(|data: &AppState, _| format!("{}", data.status));

    let send_button = Button::new("Send")
        .on_click(|_ctx, data: &mut AppState, _env| {
            // Implement sending logic here
            data.status = "Sending...".to_string();
        });

    let receive_button = Button::new("Receive")
        .on_click(|_ctx, data: &mut AppState, _env| {
            // Implement receiving logic here
            data.status = "Receiving...".to_string();
        });

    Flex::column()
        .with_child(file_path_input)
        .with_child(send_button)
        .with_child(receive_button)
        .with_child(status_label)
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Xender Rust")
        .window_size((400.0, 200.0));

    let initial_state = AppState {
        file_path: "".into(),
        status: "Idle".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
