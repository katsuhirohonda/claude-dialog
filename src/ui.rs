use colored::*;

pub struct UI;

impl UI {
    pub fn print_welcome(system_prompt_info: &str, model: Option<&String>) {
        println!("{}", "━".repeat(60).blue());
        println!("{}", "Claude Dialog Shell".yellow().bold());
        println!("System Prompt: {}", system_prompt_info.green());
        if let Some(model) = model {
            println!("Model: {}", model.green());
        }
        println!("{}", "━".repeat(60).blue());
        println!();
        println!("Type 'exit' or 'quit' to end the conversation");
        println!();
    }
    
    pub fn print_user_prompt() {
        print!("{} ", "You>".green());
    }
    
    pub fn print_claude_prompt() {
        println!("{}", "Claude>".blue());
    }
    
    pub fn print_exit_message() {
        println!("{}", "Exiting conversation...".yellow());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_creation() {
        // This is mainly to ensure UI struct can be created
        let _ui = UI;
    }
}