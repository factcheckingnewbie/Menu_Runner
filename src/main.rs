use iced::{
    button, Button, Column, Element, Application, Command, Settings, Text, Length,
};
use iced::executor;

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(String),
    ScriptFinished(Result<(), String>),
}

struct GuiApp {
    // Each tuple contains the argument text and a Button state used by Iced.
    buttons: Vec<(String, button::State)>,
    // A field to show the result (or error) of the last script execution.
    last_message: Option<String>,
}

impl Application for GuiApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    // On startup, read the config file and build one button per non-empty line.
    fn new(_flags: ()) -> (GuiApp, Command<Message>) {
        let mut buttons = Vec::new();
        match std::fs::read_to_string("config.txt") {
            Ok(content) => {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        buttons.push((trimmed.to_string(), button::State::new()));
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading config file: {}", err);
            }
        }
        (
            GuiApp {
                buttons,
                last_message: None,
            },
            Command::none(),
        )
    }

    // The window title.
    fn title(&self) -> String {
        "scripy Menu Runner".to_string()
    }

    // Handle events.
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // When a button is pressed, run the Bash script asynchronously.
            Message::ButtonPressed(arg) => {
                Command::perform(run_script(arg.clone()), Message::ScriptFinished)
            }
            // When the script finishes, record whether it succeeded or failed.
            Message::ScriptFinished(result) => {
                self.last_message = Some(match result {
                    Ok(_) => "Script executed successfully.".into(),
                    Err(err) => err,
                });
                Command::none()
            }
        }
    }

    // Render the UI: A column of buttons (one per argument) plus the last execution message.
    fn view(&mut self) -> Element<Message> {
        let mut content = Column::new()
            .padding(20)
            .spacing(10)
            .width(Length::Fill);

        for (arg, state) in self.buttons.iter_mut() {
            let btn = Button::new(state, Text::new(arg))
                .padding(10)
                .on_press(Message::ButtonPressed(arg.clone()));
            content = content.push(btn);
        }

        if let Some(ref msg) = self.last_message {
            content = content.push(Text::new(msg).size(16));
        }
        content.into()
    }
}

// This asynchronous function launches a Bash shell that runs 'script.sh' using the provided argument.
// It waits for the process to complete and returns Ok(()) on success or an error message on failure.
async fn run_script(arg: String) -> Result<(), String> {
    let mut child = std::process::Command::new("bash")
        .arg("script.sh")
        .arg(arg)
        .spawn()
        .map_err(|e| format!("Failed to launch script: {}", e))?;

    child
        .wait()
        .map_err(|e| format!("Error waiting for script: {}", e))?;
    Ok(())
}

fn main() {
    GuiApp::run(Settings::default());
}

