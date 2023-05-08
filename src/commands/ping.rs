use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

pub fn run(_option: &[CommandDataOption]) -> String {
    String::from("pong!")
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("a ping command.")
}
