mod mira;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group, hook},
        Args, CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready, prelude::UserId},
    prelude::TypeMapKey,
};
use std::{collections::HashMap, fs};

pub const KRZYCZ: &'static str = "krzycz `:kobler kurwa` by otrzymać wsparcie.";

#[hook]
async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            &format!(
                "nie rozpoznano komendy `{}`. {}",
                unknown_command_name, KRZYCZ
            ),
        )
        .await;
}

struct ArchetypeHolder;

impl TypeMapKey for ArchetypeHolder {
    type Value = HashMap<UserId, mira::Archetype>;
}

#[group]
#[prefixes("kobler", "k")]
#[commands(kurwa, prawzór, rzut, zanik)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        match ctx.cache.guilds().await.pop() {
            Some(guild) => {
                match ctx.cache.guild_channels(guild).await {
                    Some(channels) => {
                        for channel in channels.values() {
                            if channel.is_text_based() {
                                let message = channel
                                    .send_message(&ctx, |m| {
                                        m.content(format!("kobler aktywny. {}", KRZYCZ))
                                    })
                                    .await;
                                if let Err(why) = message {
                                    println!("błąd wiadomości: {:?}", why);
                                };
                                break;
                            }
                        }
                    }
                    None => println!("gildia nie posiada kanałów"),
                };
            }
            None => println!("nie znaleziono aktywnej gildii"),
        };
        println!("{} aktywny", ready.user.name);
    }
}

fn token() -> String {
    fs::read_to_string("auth-token.secret").expect("nimożność odczytania tokenu")
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(":"))
        .unrecognised_command(unknown_command)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token().trim())
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<ArchetypeHolder>(HashMap::default())
        .await
        .expect("nimożność utworzenia klienta");

    if let Err(why) = client.start().await {
        println!("błąd podczas biegu klienta: {:?}", why);
    }
}

#[command]
async fn kurwa(ctx: &Context, msg: &Message) -> CommandResult {
    let help = fs::read_to_string("readme.md").expect("miemożność odczytania pomocy");
    msg.reply(ctx, help).await?;

    Ok(())
}

fn prawzór_gracza(
    holder: &HashMap<UserId, mira::Archetype>,
    user: &UserId,
) -> Option<mira::Archetype> {
    match holder.get(user) {
        Some(wzór) => Some(*wzór),
        None => None,
    }
}

#[command]
async fn prawzór(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let holder = data
        .get_mut::<ArchetypeHolder>()
        .expect("spodziewano się ArchetypeHolder w TypeMap.");
    match args.current() {
        Some(arg) => match mira::Archetype::parse(arg) {
            Some(wzór) => {
                let entry = holder
                    .entry(msg.author.id)
                    .or_insert(mira::Archetype::Fighter);
                *entry = wzór;
                msg.reply(ctx, format!("zapisano prawzór {}.", wzór))
                    .await?;
            }
            None => {
                msg.reply(ctx, format!("argument niepoprawny. {}", KRZYCZ))
                    .await?;
            }
        },
        None => {
            match prawzór_gracza(&holder, &msg.author.id) {
                Some(wzór) => msg.reply(ctx, format!("twój prawzór to {}", wzór)).await?,
                None => msg.reply(ctx, "nie posiadasz prawzoru.").await?,
            };
        }
    };

    Ok(())
}

#[command]
async fn rzut(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let holder = data
        .get_mut::<ArchetypeHolder>()
        .expect("spodziewano się ArchetypeHolder w TypeMap.");
    match prawzór_gracza(&holder, &msg.author.id) {
        Some(wzór) => {
            let mut field = mira::Field::Untrained;
            let mut tool = mira::Tool::Bare;
            let mut error = "".to_string();
            while !args.is_empty() {
                match args
                    .current()
                    .expect("było sprawdzone czy pusty")
                    .to_lowercase()
                {
                    arg if arg == "-s" => field = mira::Field::Trained,
                    arg if arg == "-sb" => field = mira::Field::Proficient,
                    arg if arg == "-n" => {
                        args.advance();
                        match mira::Tool::parse_tool(args.current()) {
                            Ok(t) => tool = t,
                            Err(e) => error += &e,
                        };
                    }
                    arg if arg == "-b" => {
                        args.advance();
                        match mira::Tool::parse_weapon(args.current()) {
                            Ok(t) => tool = t,
                            Err(e) => error += &e,
                        };
                    }
                    arg => error += &format!("niepoprawny argument: {}. ", arg),
                };
                args.advance();
            }
            if error == "" {
                msg.reply(ctx, mira::dice(&wzór, &field, &tool)).await?;
            } else {
                msg.reply(ctx, format!("{}", error + KRZYCZ)).await?;
            }
        }
        None => {
            msg.reply(ctx, format!("nie posiadasz prawzoru. {}", KRZYCZ))
                .await?;
        }
    };

    Ok(())
}

#[command]
async fn zanik(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut durability: usize = 0;
    let mut quality = mira::Quality::Decent;
    let mut error = "".to_string();
    match args.current() {
        Some(dur) => {
            match dur.parse::<usize>() {
                Ok(n) => durability = n,
                Err(err) => error += &format!("niepoprawny argument trwałości: {}, {}. ", dur, err),
            };

            while !args.is_empty() {
                match args
                    .current()
                    .expect("było sprawdzone czy pusty")
                    .to_lowercase()
                {
                    arg if arg == "-m" => quality = mira::Quality::Fine,
                    arg if arg == "-p" => quality = mira::Quality::Decent,
                    arg if arg == "-l" => quality = mira::Quality::Crude,
                    arg => error += &format!("niepoprawny argument jakości: {}. ", arg),
                };
                args.advance();
            }
        }
        None => error += &format!("nie podano trwałości. "),
    };

    if error == "" {
        msg.reply(ctx, mira::zanik(durability, &quality)).await?;
    } else {
        msg.reply(ctx, format!("{}", error + KRZYCZ)).await?;
    }

    Ok(())
}
