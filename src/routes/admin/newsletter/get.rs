use actix_web::{HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn send_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap()
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Send a newsletter issue</title>
            </head>
            <body>
                {msg_html}
                <form action="/admin/newsletter method="post">
                    <label>Title
                        <input
                            type="text"
                            name="newsletter_title"
                        >
                    </label>
                    <br>
                    <label>Body
                        <input
                            type="text"
                            name="newsletter_body"
                        >
                    </label>
                    <br>
                    <button type="submit">Send newsletter</button>
                </form>
                <p><a href="/admin/dashboard">&lt;- Back</a></p>
            </body>
            </html>"#
        )))
}
