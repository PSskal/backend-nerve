use actix_web::{HttpRequest, HttpResponse, web, Error};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;

pub async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    actix_web::rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    let response_text = format!("Mensaje recibido: {}", text);
                    if let Err(_) = session.text(response_text).await {
                        break;
                    }
                }
                Ok(AggregatedMessage::Binary(bin)) => {
                    if let Err(_) = session.binary(bin).await {
                        break;
                    }
                }
                Ok(AggregatedMessage::Ping(msg)) => {
                    if let Err(_) = session.pong(&msg).await {
                        break;
                    }
                }
                Ok(AggregatedMessage::Close(_)) => {
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(res)
}