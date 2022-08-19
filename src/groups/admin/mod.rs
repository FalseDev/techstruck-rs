mod register;

pub(crate) fn commands() -> Vec<poise::Command<crate::Data, crate::Error>> {
    vec![register::register()]
}
