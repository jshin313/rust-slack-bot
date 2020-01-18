# rust-slack-bot
Greets anyone who recently joins the Slack workspace. Made for a Google Code-In task.

## Setup

To get an API Key, you can start by creating a Slack Bot [here](https://my.slack.com/services/new/bot). Then under "Integration Settings", you will find the API Token.
Unforunately, free Slack accounts can only have up to 10 integrations (bots and apps are counted as types of integrations). This bot was made for the CCExtractor Slack group, so since I was fairly late in making my bot, many other users had already added their bots to Slack and used up that limit.
So I had to create another user to mimic a bot instead of creating a real bot. If you want to be able to control your user, you can try getting a user token to control a user instead of a bot token. The user token is a legacy token, so it's now oudated; use it at your own risk. If you still want to use it, you can get legacy token [here](https://api.slack.com/custom-integrations/legacy-tokens).

## Usage
`cargo run <api-token>`
or when using the Dockerfile, make sure to replace <api-key> with the proper value.

## Credits
The code uses the [slack-rs](https://github.com/slack-rs/slack-rs) library extensively and is based upon the example [here](https://github.com/slack-rs/slack-rs/blob/master/examples/slack_example.rs).
