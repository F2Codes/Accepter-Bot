use teloxide::{
    dispatching::update_listeners::Listener,
    prelude::*,
    types::{ChatJoinRequest, ChatMemberUpdated, InlineKeyboardButton, InlineKeyboardMarkup},
};

use std::env;

// Replace with your channel ID
const CHANNEL_ID: i64 = -1001234567890;

#[tokio::main]
async fn main() {
    // Load bot token from environment variable
    let bot = Bot::from_env().auto_send();

    println!("Bot is running… 🚀");

    Dispatcher::builder(bot.clone(), Update::filter_all())
        .default_handler(|upd| async move {
            log::info!("Unhandled update: {:?}", upd);
        })

        // 1) Auto-approve join requests
        .chat_join_request_handler(handle_join_request)

        // 2) Send welcome message inside channel
        .chat_member_handler(handle_chat_member)

        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}


// -------------------------------------------------------------
// 1) AUTO-ACCEPT JOIN REQUEST + SEND PRIVATE WELCOME MESSAGE
// -------------------------------------------------------------
async fn handle_join_request(
    bot: AutoSend<Bot>,
    req: ChatJoinRequest,
) -> ResponseResult<()> {
    let user = req.from.clone();
    let name = user.first_name.clone().unwrap_or("Friend".into());

    // Approve request
    bot.approve_chat_join_request(req.chat.id, user.id).await?;

    // Create inline button to join the channel
    let invite_link = req.invite_link.unwrap().invite_link;

    let keyboard = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::url("🔵 Enter Channel", invite_link),
    ]]);

    let text = format!(
        "❗❗ *Welcome aboard!*  
Your request has been *accepted* 🎉

Press the button below to enter the channel 👇"
    );

    // Send DM (ignore if user hasn't started bot)
    let _ = bot
        .send_message(user.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboard)
        .await;

    Ok(())
}


// -------------------------------------------------------------
// 2) CHANNEL WELCOME MESSAGE (same style you requested)
// -------------------------------------------------------------
async fn handle_chat_member(
    bot: AutoSend<Bot>,
    update: ChatMemberUpdated,
) -> ResponseResult<()> {
    let new_member = update.new_chat_member;

    if update.chat.id != CHANNEL_ID {
        return Ok(());
    }

    if new_member.is_member() && !new_member.user.is_bot {
        let name = new_member.user.first_name.clone().unwrap_or("Friend".into());

        let text = format!(
            "❗❗
- Your request to join the **channel** has been accepted!

Your membership is confirmed ✅  
Welcome, {} 🌟

👤 Admin: @ADMINS_USERNAME",
            name
        );

        bot.send_message(CHANNEL_ID, text)
            .parse_mode(ParseMode::Markdown)
            .await?;
    }

    Ok(())
}
