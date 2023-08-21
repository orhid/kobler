/*
#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo,
    // clippy::expect_used,
)]
*/
#![allow(clippy::incorrect_partial_ord_impl_on_ord_type)] // bug in derivative
#![feature(let_chains)]
#![feature(extract_if)]
#![feature(hash_extract_if)]

use crate::parser::Arg;
use derivative::Derivative;
use itertools::Itertools;
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
use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
    sync::Arc,
};
use strsim::damerau_levenshtein as dist;

/* constants and modules */

mod parser;
mod zug;
pub const KRZYCZ: &str = "krzycz `:kobler kurwa` by otrzymać wsparcie.";

/* helper functions */

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

/* helper types */

struct WzorzecHolder;

impl TypeMapKey for WzorzecHolder {
    type Value = HashMap<UserId, zug::Wzorzec>;
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BrońGracza {
    #[derivative(
        PartialEq = "ignore",
        Hash = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    broń: zug::Broń,

    aktywna: bool,
    nazwa: Arc<str>,
}

impl fmt::Display for BrońGracza {
    #[allow(clippy::match_bool)] // i think this is more readable
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) : {}, {})",
            self.nazwa,
            match self.aktywna {
                true => "wybrana",
                false => "schowana",
            },
            self.broń.zasięg_str(),
            self.broń.waga_str()
        )
    }
}

type BronieGracza = HashSet<BrońGracza>;

struct BronieGraczaHolder;

impl TypeMapKey for BronieGraczaHolder {
    type Value = HashMap<UserId, BronieGracza>;
}

/* mięsko */

#[group]
#[prefixes("kobler", "k")]
#[commands(kurwa, wzorzec, broń, próba, bitwa, zanik)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(guild) = ctx.cache.guilds().await.pop() {
            if let Some(channels) = ctx.cache.guild_channels(guild).await {
                // maybe there is a way to find a better channel to post in
                if let Some(channel) = channels.values().find(|channel| channel.is_text_based()) {
                    let message = channel
                        .send_message(&ctx, |m| m.content(format!("kobler aktywny. {}", KRZYCZ)))
                        .await;
                    if let Err(why) = message {
                        println!("błąd wiadomości: {:?}", why);
                    };
                }
            } else {
                println!("gildia nie posiada kanałów");
            }
        } else {
            println!("nie znaleziono aktywnej gildii");
        }
        println!("{} aktywny", ready.user.name);
    }
}

fn token() -> Result<String, impl std::error::Error> {
    fs::read_to_string("auth-token.secret")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(":"))
        .unrecognised_command(unknown_command)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token()?.trim())
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<WzorzecHolder>(HashMap::default())
        .await?;

    client.start().await?;
    Ok(())
}

/* komendy */

/* ## kurwa */

#[command]
async fn kurwa(ctx: &Context, msg: &Message) -> CommandResult {
    let help = fs::read_to_string("readme.md")?;
    msg.reply(ctx, help).await?;

    Ok(())
}

/* ## wzorzec */

#[command]
#[allow(clippy::significant_drop_tightening)] // compiler errors if sugestion followed
async fn wzorzec(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let holder = data
        .get_mut::<WzorzecHolder>()
        .expect("spodziewano się WzorzecHolder w TypeMap.");

    let args = Arg::try_parse(args)?;
    match args.first() {
        Some(arg) => {
            if let Some(wzorzec) = zug::Wzorzec::try_parse(arg) {
                if let Some(entry) = holder.insert(msg.author.id, wzorzec) {
                    msg.reply(
                        ctx,
                        format!("zamieniono wzorzec z {} na {}.", entry, wzorzec),
                    )
                    .await?
                } else {
                    msg.reply(ctx, format!("zapisano wzorzec {}.", wzorzec))
                        .await?
                }
            } else {
                msg.reply(ctx, format!("argument niepoprawny. {}", KRZYCZ))
                    .await?
            }
        }
        None => {
            if let Some(wzorzec) = holder.get(&msg.author.id) {
                msg.reply(ctx, format!("twój wzorzec to {}", wzorzec))
                    .await?
            } else {
                msg.reply(ctx, "nie posiadasz prawzoru.").await?
            }
        }
    };

    Ok(())
}

/* ## broń */

async fn broń_helper<F>(
    ctx: &Context,
    msg: &Message,
    args: Vec<Arg>,
    holder: &mut <BronieGraczaHolder as TypeMapKey>::Value,
    action: F,
    msg_on_success: &str,
    readd: bool,
) -> CommandResult
where
    F: Fn(&mut BrońGracza) + Send,
{
    if let Some(options) = args
        .iter()
        .filter_map(|arg| match arg {
            Arg::Short('n', options) => Some(options),
            Arg::Long(param, options) => {
                if dist(param, "nazwa") < 3 {
                    Some(options)
                } else {
                    None
                }
            }
            _ => None,
        })
        .last() && let Some(nazwa) = options.last()
    {
        if let Some(bronie) = holder.get_mut(&msg.author.id) {
            if let Some(mut broń) = bronie
                .extract_if(|broń| dist(&broń.nazwa, nazwa) < 3)
                .last()
            {
                action(&mut broń);
                if readd {bronie.insert(broń);}
                msg.reply(ctx, msg_on_success).await?;
            } else {
                msg.reply(ctx, "nie posiadasz broni o podanej nazwie.")
                    .await?;
            }
        } else {
            msg.reply(ctx, "nie posiadasz żadnej broni.").await?;
        }
    } else {
        msg.reply(ctx, format!("nie podano argumentu nazwy. {}", KRZYCZ))
            .await?;
    }

    Ok(())
}

async fn broń_dodaj(
    ctx: &Context,
    msg: &Message,
    args: Vec<Arg>,
    holder: &mut <BronieGraczaHolder as TypeMapKey>::Value,
) -> CommandResult {
    if let Some(options) = args
        .iter()
        .filter_map(|arg| match arg {
            Arg::Short('n', options) => Some(options),
            Arg::Long(param, options) => {
                if dist(param, "nazwa") < 3 {
                    Some(options)
                } else {
                    None
                }
            }
            _ => None,
        })
        .last() && let Some(nazwa) = options.last()
    {
        if let Ok(broń) = zug::Broń::try_parse(&args)  {
            let broń = BrońGracza {nazwa: Arc::from(nazwa.as_str()), aktywna: false, broń};
            if let Some(bronie) = holder.get_mut(&msg.author.id) {
                bronie.insert(broń);
            } else {
                let mut bronie = HashSet::new();
                bronie.insert(broń);
                holder.insert(msg.author.id, bronie);
            }
            msg.reply(ctx, "dodano broń.").await?;
        } else {
            msg.reply(ctx, format!("podano niepoprawny argument broni. {}", KRZYCZ)).await?;
        }
    } else {
        msg.reply(ctx, format!("nie podano argumentu nazwy. {}", KRZYCZ))
            .await?;
    }

    Ok(())
}

async fn broń_wybierz(
    ctx: &Context,
    msg: &Message,
    args: Vec<Arg>,
    holder: &mut <BronieGraczaHolder as TypeMapKey>::Value,
) -> CommandResult {
    broń_helper(
        ctx,
        msg,
        args,
        holder,
        |broń| broń.aktywna = true,
        "wybrano broń.",
        true,
    )
    .await
}

async fn broń_schowaj(
    ctx: &Context,
    msg: &Message,
    args: Vec<Arg>,
    holder: &mut <BronieGraczaHolder as TypeMapKey>::Value,
) -> CommandResult {
    broń_helper(
        ctx,
        msg,
        args,
        holder,
        |broń| broń.aktywna = false,
        "schowano broń.",
        true,
    )
    .await
}

async fn broń_usuń(
    ctx: &Context,
    msg: &Message,
    args: Vec<Arg>,
    holder: &mut <BronieGraczaHolder as TypeMapKey>::Value,
) -> CommandResult {
    broń_helper(ctx, msg, args, holder, |_| {}, "usunięto broń.", false).await
}

#[command]
#[allow(clippy::significant_drop_tightening)] // compiler errors if sugestion followed
async fn broń(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let holder = data
        .get_mut::<BronieGraczaHolder>()
        .expect("spodziewano się WzorzecHolder w TypeMap.");

    let args = Arg::try_parse(args)?;
    match args.first() {
        Some(arg) => if let Arg::Plain(cmd) = arg {
            match cmd.as_str() {
                "dodaj" => broń_dodaj(ctx, msg, args, holder).await?,
                "wybierz" => broń_wybierz(ctx, msg, args, holder).await?,
                "schowaj" => broń_schowaj(ctx, msg, args, holder).await?,
                "usuń" => broń_usuń(ctx, msg, args, holder).await?,
                _ => {
                    msg.reply(ctx, format!("argument niepoprawny. {}", KRZYCZ)).await?;
                },
            }
        } else {
            msg.reply(ctx, format!("argument niepoprawny. {}", KRZYCZ)).await?;
        },
        None => {
            if let Some(bronie) = holder.get(&msg.author.id) && !bronie.is_empty() {
                msg.reply(ctx,
                          bronie
                            .iter()
                            .sorted()
                            .map(|broń_gracza| format!("```\n{broń_gracza}\n```"))
                            .join("\n")
                    ).await?;
            } else {
                msg.reply(ctx, "nie posiadasz żadnej broni.").await?;
            }
        },
    }

    Ok(())
}

/* ## próba */
#[command]
#[allow(clippy::significant_drop_tightening)] // compiler errors if sugestion followed
async fn próba(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.write().await;
    let holder = data
        .get::<WzorzecHolder>()
        .expect("spodziewano się WzorzecHolder w TypeMap.");

    if let Some(wzorzec) = holder.get(&msg.author.id) {
        let args = Arg::try_parse(args)?;
        msg.reply(
            ctx,
            zug::próba(
                *wzorzec,
                args.iter()
                    .filter_map(zug::Fach::try_parse)
                    .last()
                    .unwrap_or_default(),
                args.iter().filter_map(zug::Narzędzie::try_parse).last(),
            ),
        )
        .await?;
    } else {
        msg.reply(ctx, "nie posiadasz prawzoru.").await?;
    }

    Ok(())
}

/* ## bitwa */

#[command]
#[allow(clippy::significant_drop_tightening)] // compiler errors if sugestion followed
async fn bitwa(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.write().await;
    let holder_wzorzec = data
        .get::<WzorzecHolder>()
        .expect("spodziewano się WzorzecHolder w TypeMap.");
    let holder_broń = data
        .get::<BronieGraczaHolder>()
        .expect("spodziewano się WzorzecHolder w TypeMap.");

    if let Some(wzorzec) = holder_wzorzec.get(&msg.author.id) {
        let args = Arg::try_parse(args)?;
        let mod_pos = args
            .iter()
            .filter_map(|arg| match arg {
                Arg::Short('p', options) => Some(options),
                Arg::Long(param, options) => {
                    if dist(param, "plus") < 3 {
                        Some(options)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .filter_map(|options| options.last()?.parse::<isize>().ok())
            .sum::<isize>();
        let mod_neg = args
            .iter()
            .filter_map(|arg| match arg {
                Arg::Short('m', options) => Some(options),
                Arg::Long(param, options) => {
                    if dist(param, "minus") < 3 {
                        Some(options)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .filter_map(|options| options.last()?.parse::<isize>().ok())
            .sum::<isize>();

        let kurwa = BronieGracza::new(); // necessary to make the next line work
        msg.reply(
            ctx,
            zug::bitwa(
                *wzorzec,
                holder_broń
                    .get(&msg.author.id)
                    .unwrap_or(&kurwa)
                    .iter()
                    .filter(|broń| broń.aktywna)
                    .map(|broń| broń.broń),
                mod_pos - mod_neg,
            ),
        )
        .await?;
    } else {
        msg.reply(ctx, "nie posiadasz prawzoru.").await?;
    }

    Ok(())
}

/* ## zanik */

#[command]
async fn zanik(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args = Arg::try_parse(args)?;
    if let Some(arg) = args.first() {
        if let Arg::Plain(trwałość_str) = arg && let Ok(trwałość) = trwałość_str.parse::<usize>() {
            if let Some(jakość) = args.last().map(zug::Narzędzie::try_parse).unwrap_or_default()  {
                if let Ok(message) =zug::zanik(trwałość, jakość) { 
                msg.reply(ctx, message).await?;
                } else {
                msg.reply(ctx, format!("podano niepoprawną jakość. {}", KRZYCZ)).await?;
                }
            } else {
                msg.reply(ctx, format!("podano niepoprawną jakość. {}", KRZYCZ)).await?;
            }
        } else {
            msg.reply(ctx, format!("podano niepoprawną trwałość. {}", KRZYCZ)).await?;
        }
    } else {
        msg.reply(ctx, format!("nie podano trwałości. {}", KRZYCZ))
            .await?;
    }
    Ok(())
}
