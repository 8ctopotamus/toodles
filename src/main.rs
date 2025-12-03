use color_eyre::eyre::{ Ok, Result };
use ratatui::{ 
    crossterm::{
        event::{ self, Event },
        // terminal,
    }, 
    DefaultTerminal, Frame
};

fn main() -> Result<()> {
    println!("Hello, world!");
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // render
        terminal.draw(render);
        // input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }    
    Ok(())
}

fn render(Frame frame) {

}