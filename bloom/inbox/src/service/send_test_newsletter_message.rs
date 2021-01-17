use super::SendTestNewsletterMessageInput;
use crate::{entities::NewsletterMessage, Error, Service};
use kernel::Actor;
use stdx::mail;

impl Service {
    pub async fn send_test_newsletter_message(
        &self,
        actor: Actor,
        input: SendTestNewsletterMessageInput,
    ) -> Result<NewsletterMessage, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let message = self
            .repo
            .find_newsletter_message_by_id(&self.db, input.message_id)
            .await?;

        self.kernel_service
            .check_namespace_membership(&self.db, actor.id, message.namespace_id)
            .await
            .map_err(|_| Error::NewsletterMessageNotFound)?;

        let to = mail::Address {
            name: actor.name,
            address: actor.email,
        };
        let job = kernel::domain::messages::Message::InboxSendNewsletterMessage {
            message_id: message.id,
            to,
            is_test: true,
        };
        let _ = self.queue.push(job, None).await; // TODO: log error?

        Ok(message)
    }
}
