const TelegramBot = require("node-telegram-bot-api");

// ====== CONFIG ======
const TOKEN = "YOUR_TELEGRAM_BOT_API"; 
const ADMIN_USERNAME = "@YOUR_ADMINS_USERNAME"; 
const CHANNEL_INVITE_LINK = "YOUR_CHANNELS_PRIVATE_LINK"; 
// ======================

const bot = new TelegramBot(TOKEN, { polling: true });

console.log("Bot Accepter is running...");

// When a join request arrives
bot.on("chat_join_request", async (req) => {
  const chatId = req.chat.id;
  const userId = req.from.id;
  const firstName = req.from.first_name || "User";

  try {
    // Approve request
    await bot.approveChatJoinRequest(chatId, userId);

    // Inline button
    const opts = {
      reply_markup: {
        inline_keyboard: [
          [
            {
              text: "🔵 Enter Channel",
              url: CHANNEL_INVITE_LINK
            }
          ]
        ]
      },
      parse_mode: "Markdown"
    };

    // DM message
    const welcomeText =
      `🎉 *Welcome, ${firstName}!* \n\n` +
      `Your request to join our channel has been *approved*. 🚀\n` +
      `Enjoy your stay!\n\n` +
      `👤 *Admin:* ${ADMIN_USERNAME}`;

    await bot.sendMessage(userId, welcomeText, opts);

  } catch (err) {
    console.log("Error:", err.message);
  }
});
