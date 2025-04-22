pub enum EmailNotifierError {}

pub struct EmailMessage {}

pub struct EmailResult {}

pub trait EmailNotifier {
    fn notify(&self, message: EmailMessage) -> Result<EmailResult, EmailNotifierError>;
}
