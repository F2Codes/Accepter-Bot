from telegram import Update, InlineKeyboardButton, InlineKeyboardMarkup
from telegram.ext import (
    Application,
    ChatJoinRequestHandler,
    ChatMemberHandler,
    ContextTypes,
)

TOKEN = "YOUR_BOT_TOKEN_HERE"
CHANNEL_ID = -1001234567890  # Your private channel ID


# -------------------------------------------------
# 1) AUTO-APPROVE JOIN REQUEST + SEND DM WELCOME
# -------------------------------------------------
async def handle_join_request(update: Update, context: ContextTypes.DEFAULT_TYPE):
    req = update.chat_join_request
    user = req.from_user
    invite_link = req.invite_link.invite_link

    # Accept the request
    await context.bot.approve_chat_join_request(
        chat_id=req.chat.id, user_id=user.id
    )

    # Welcome message (Direct Message)
    keyboard = InlineKeyboardMarkup([
        [InlineKeyboardButton("🔵 Enter Channel", url=invite_link)]
    ])

    message = (
        "❗❗ **Welcome aboard!**\n"
        "Your request to join the channel has been **approved** 🎉\n\n"
        "Tap the button below to enter 👇"
    )

    try:
        await context.bot.send_message(
            chat_id=user.id,
            text=message,
            parse_mode="Markdown",
            reply_markup=keyboard
        )
    except:
        # In case the user has never started the bot
        pass


# -------------------------------------------------
# 2) SEND WELCOME MESSAGE INSIDE THE CHANNEL
# -------------------------------------------------
async def handle_member_update(update: Update, context: ContextTypes.DEFAULT_TYPE):
    data = update.chat_member
    new_member = data.new_chat_member
    user = new_member.user

    # Trigger only when the user truly joins the channel
    if (
        data.chat.id == CHANNEL_ID
        and new_member.status == "member"
        and not user.is_bot
    ):
        name = user.first_name or "Friend"

        message = (
            "❗❗\n"
            "- Your request to join the **channel** has been approved!\n\n"
            f"Welcome, {name}! Glad to have you here 🌟\n\n"
            "👤 Admin: @ADMINS_USERNAME"
        )

        await context.bot.send_message(
            chat_id=CHANNEL_ID,
            text=message,
            parse_mode="Markdown"
        )


# -------------------------------------------------
# 3) START BOT
# -------------------------------------------------
def main():
    app = Application.builder().token(TOKEN).build()

    # Auto-approve join requests
    app.add_handler(ChatJoinRequestHandler(handle_join_request))

    # Welcome message inside the channel
    app.add_handler(ChatMemberHandler(
        handle_member_update,
        ChatMemberHandler.CHAT_MEMBER
    ))

    print("Bot is running...")
    app.run_polling()


if __name__ == "__main__":
    main()
