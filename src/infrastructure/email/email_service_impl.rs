use crate::{
    domain::services::EmailService,
    shared::utils::constants::{BASE_URL, FROM_EMAIL, SMTP_PASSWORD, SMTP_SERVER, SMTP_USERNAME},
};
use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::Credentials,
};

pub struct SmtpEmailService {
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    base_url: String,
}

impl SmtpEmailService {
    pub fn new() -> Result<Self, String> {
        let from_email = FROM_EMAIL.clone();
        let base_url = BASE_URL.clone();

        let credentials = Credentials::new(SMTP_USERNAME.clone(), SMTP_PASSWORD.clone());

        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&SMTP_SERVER)
            .map_err(|err| format!("Failed to create SMTP transport: {}", err))?
            .credentials(credentials)
            .build();

        Ok(SmtpEmailService {
            smtp_transport,
            from_email,
            base_url,
        })
    }
}

#[async_trait]
impl EmailService for SmtpEmailService {
    async fn send_activation_email(
        &self,
        to_email: &str,
        username: &str,
        activation_token: &str,
    ) -> Result<(), String> {
        let activation_link = format!("{}/api/activate?token={}", self.base_url, activation_token);

        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0; padding: 0; font-family: Arial, sans-serif; background-color: #f4f4f4;">
    <table width="100%" cellpadding="0" cellspacing="0" style="background-color: #f4f4f4; padding: 20px 0;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="padding: 40px 30px;">
                            <h1 style="color: #333333; font-size: 24px; margin: 0 0 20px 0;">Hello {username},</h1>

                            <p style="color: #666666; font-size: 16px; line-height: 1.6; margin: 0 0 20px 0;">
                                Thanks for creating an account with <strong>Kanblast</strong>! Before you can start using the app, we need to confirm that this email address belongs to you.
                            </p>

                            <p style="color: #666666; font-size: 16px; line-height: 1.6; margin: 0 0 30px 0;">
                                Please click the button below to activate your account. This is a quick, one-time step to keep your account secure.
                            </p>

                            <table width="100%" cellpadding="0" cellspacing="0" style="margin: 0 0 30px 0;">
                                <tr>
                                    <td align="center">
                                        <a href="{activation_link}" style="display: inline-block; padding: 14px 40px; background-color: #4CAF50; color: #ffffff; text-decoration: none; border-radius: 4px; font-size: 16px; font-weight: bold;">Activate My Account</a>
                                    </td>
                                </tr>
                            </table>

                            <p style="color: #666666; font-size: 14px; line-height: 1.6; margin: 0 0 10px 0;">
                                If the button doesn't work, you can copy and paste this URL into your browser:
                            </p>

                            <p style="color: #4CAF50; font-size: 14px; line-height: 1.6; margin: 0 0 30px 0; word-break: break-all;">
                                {activation_link}
                            </p>

                            <p style="color: #666666; font-size: 16px; line-height: 1.6; margin: 0 0 20px 0;">
                                Once confirmed, you'll be ready to go.
                            </p>

                            <p style="color: #666666; font-size: 16px; line-height: 1.6; margin: 0;">
                                Best regards,<br>
                                <strong>Kanblast Team</strong>
                            </p>

                            <hr style="border: none; border-top: 1px solid #eeeeee; margin: 30px 0;">

                            <p style="color: #999999; font-size: 12px; line-height: 1.6; margin: 0;">
                                If you did not create an account, please ignore this email.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#,
            username = username,
            activation_link = activation_link
        );

        // Plain text fallback
        let text_body = format!(
            r#"Hello {username},

Thanks for creating an account with Kanblast! Before you can start using the app, we need to confirm that this email address belongs to you.

Please click the link below to activate your account. This is a quick, one-time step to keep your account secure.

{activation_link}

Once confirmed, you'll be ready to go.

Best regards,
Kanblast Team

---
If you did not create an account, please ignore this email."#,
            username = username,
            activation_link = activation_link
        );

        let email = Message::builder()
            .from(
                self.from_email
                    .parse()
                    .map_err(|err| format!("Invalid from email: {}", err))?,
            )
            .to(to_email
                .parse()
                .map_err(|err| format!("Invalid to email: {}", err))?)
            .subject("Activate your Kanblast account")
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(text_body),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(html_body),
                    ),
            )
            .map_err(|err| format!("Failed to build email: {}", err))?;

        self.smtp_transport
            .send(email)
            .await
            .map_err(|err| format!("Failed to send email: {}", err))?;

        Ok(())
    }
}
