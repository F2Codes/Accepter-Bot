package main

import (
	"log"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

const (
	TOKEN      = "YOUR_BOT_TOKEN_HERE"
	CHANNEL_ID = -1001234567890 // your private channel ID
)

func main() {
	bot, err := tgbotapi.NewBotAPI(TOKEN)
	if err != nil {
		log.Panic(err)
	}

	bot.Debug = false
	log.Println("Bot is running...")

	u := tgbotapi.NewUpdate(0)
	u.Timeout = 10

	updates := bot.GetUpdatesChan(u)

	for update := range updates {

		// -----------------------------------------------------
		// 1) AUTO-ACCEPT JOIN REQUEST + SEND DM WELCOME MESSAGE
		// -----------------------------------------------------
		if update.ChatJoinRequest != nil {
			req := update.ChatJoinRequest

			user := req.From
			inviteLink := req.InviteLink.InviteLink

			// Approve join request
			_, err := bot.ApproveChatJoinRequest(req.Chat.ID, user.ID)
			if err != nil {
				log.Println("Error approving join request:", err)
			}

			// DM welcome message
			msg := tgbotapi.NewMessage(user.ID,
				"❗❗ *Welcome aboard!*\n"+
					"Your request to join the channel has been *accepted* 🎉\n\n"+
					"Click the button below to enter the channel 👇",
			)
			msg.ParseMode = "Markdown"

			// Inline button
			button := tgbotapi.NewInlineKeyboardButtonURL("🔵 Enter Channel", inviteLink)
			row := tgbotapi.NewInlineKeyboardRow(button)
			keyboard := tgbotapi.NewInlineKeyboardMarkup(row)
			msg.ReplyMarkup = keyboard

			_, _ = bot.Send(msg) // ignore error if user has not started bot manually
		}

		// -----------------------------------------------------
		// 2) SEND WELCOME IN CHANNEL AFTER USER JOINS
		// -----------------------------------------------------
		if update.MyChatMember != nil {
			continue
		}

		if update.ChatMember != nil {
			data := update.ChatMember
			newStatus := data.NewChatMember.Status
			user := data.NewChatMember.User

			// Only if user really joined the channel
			if data.Chat.ID == CHANNEL_ID && newStatus == "member" && !user.IsBot {
				name := user.FirstName

				welcome := "❗❗\n" +
					"- Your request to join the *channel* has been accepted!\n\n" +
					"Your membership is confirmed ✅\n" +
					"Welcome, " + name + " 🌟\n\n" +
					"👤 Admin: @Drackol"

				msg := tgbotapi.NewMessage(CHANNEL_ID, welcome)
				msg.ParseMode = "Markdown"

				_, _ = bot.Send(msg)
			}
		}
	}
}
