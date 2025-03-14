pub trait CommandHandler {
    async fn execute(&self);
}
