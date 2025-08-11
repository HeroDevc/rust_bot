# Minecraft anarchy chat bot
Simple minecraft anarchy chat bot written in rust 🦀 using [Azalea](https://github.com/azalea-rs/azalea).
Built on minecraft version 1.21.5

I'm still learning rust and didn't include all features.

If you want any new commands, open an issue.

Leave a star ⭐

### Features
- Anti afk
- commands like: !kills, !playtime...

### To Do
- [ ] Add discord support (commands, live chat, etc.)

### Commands
| Command | Description | Example |
| :---   | :-----: | ---: |
| help    | No help | !help |
| players | Shows how many players the bot has seen | !players |
| bestping | Shows the player with the lowest ping | !bestping |
| worstping | Shows the player with the highest ping | !worstping |
| ping | Get player's ping | !ping Steve |
| top | Shows top of something | !top playtime |
| joins | Shows how many times a player has joined the server | !joins Steve |
| leaves | Shows how many times a player has left the server | !leaves Steve |
| messages | Shows how many messages a player has sent | !messages Steve |
| curse | Curse a player | !curse Steve |
| kick | Kick a player | !kick Steve |
| ban | Ban a player | !ban Steve |
| mute | Mute a player | !mute Steve |
| firstwords | Get player's first message | !firstwords Steve |
| lastwords | Get player's last message | !lastwords Steve |
| lastkill | Get player's last kill | !lastkill Steve |
| firstkill | Get player's first kill | !firstkill Steve |
| lastdeath | Get player's last death | !lastdeath Steve |
| firstdeath | Get player's first death | !firstdeath Steve |
| seen | Shows when the bot last saw a player | !seen Steve |
| joindate | Shows when the bot first saw a player | !joindate Steve |
| kill | Make bot type "/kill" | !kill |
| kills | Shows how many kills a player has | !kills Steve |
| deaths | Shows how many deaths a player has | !deaths Steve |
| kd | Shows how many kills and deaths a player has with a kd ratio | !kd Steve |
| coords | Get bot coordinates | !coords |
| rules | Shows server rules | !rules |
| yes | Yes | !yes |
| no | No | !no |
| dupe | Dupe items | !dupe Diamonds |
| locate | Locate a player | !locate Steve |
| playtime | Get player's playtime | !playtime Steve |
| nwords | Shows how many times a player has said the n word | !nwords Steve |
| health | Shows bot's health | !health |
| food | Shows bot's food | !food |
| savemsg | Save a message | !savemsg My message |
| playmsg | Play a player's message | !playmsg Steve |
| iam | Save a message about yourself | !iam Minecraft player |
| whois | Shows a message about a player | !whois Steve |
| leak | Leak something | !leak 0 0 |
| gamemode | Set your gamemode | !gamemode Creative |
| askgod | Ask god for an answer | !askgod Can I get diamonds? |
| give | Give an item to a player | !give Steve Diamonds |
| teleport | Teleport to a location or a player | !teleport Steve |
| op | Get operator status | !op |
| kit | Get a kit | !kit pvp |
| pp | Get your pp size | !pp |
| online | Get players online in a server | !online |
| y/n | Yes or no | !y/n |
| dice | Roll a dice | !dice |
| infect | Infect a player with autism | !infect Steve |
| execute | Vote for a player's execution | !execute Steve |
| vote | Vote in an execution | !vote yes |
| jew | Check if a player is a jew | !jew Steve |
| cooltext | Apply custom font to your message | !cooltext My cool message |
| motd | Get a random motd | !motd |
| summon | Summon something | !summon Herobrine |
| setjm | Set a join message for a player | !setjm Steve Steve is here! |
| remjm | Remove a join message for a player | !remjm Steve |
| setlm | Set a leave message for a player | !setlm Steve Steve left |
| remlm | Remove a leave message for a player | !remjm Steve |
| addfaq | Add a random faq | !faq Steve has diamonds |
| faq | Get a random faq | !faq |
| q | Get 2b2t's queue size | !q |
| report | Report a player | !report Steve Cheating |
| quote | Get a random message a player has sent | !quote Steve |
| tps | Calculates tps (ticks per second) | !tps |
| task | Get a random task to complete | !task |
| bible | Get a random bible verse | !bible |

### Database Schema
"seen" table:
```sql
CREATE TABLE seen (
    player_name TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"nwords" table:
```sql
CREATE TABLE nwords (
    player_name TEXT NOT NULL,
    hard INT NOT NULL DEFAULT 0,
    soft INT NOT NULL DEFAULT 0,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"kds" table:
```sql
CREATE TABLE kds (
    player_name TEXT NOT NULL,
    kills INT NOT NULL DEFAULT 0,
    deaths INT NOT NULL DEFAULT 0,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"playtime" table:
```sql
CREATE TABLE playtime (
    player_name TEXT NOT NULL,
    seconds BIGINT NOT NULL,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"chatcount" table:
```sql
CREATE TABLE chatcount (
    player_name TEXT NOT NULL,
    count INT NOT NULL DEFAULT 0,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"joinsleaves" table:
```sql
CREATE TABLE joinsleaves (
    player_name TEXT NOT NULL,
    joins INT NOT NULL DEFAULT 0,
    leaves INT NOT NULL DEFAULT 0,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"chatlogs" table:
```sql
CREATE TABLE chatlogs (
    player_name TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    server TEXT NOT NULL
);
```
"lastkills" table:
```sql
CREATE TABLE lastkills (
    player_name TEXT NOT NULL,
    first_kill_message TEXT,
    first_kill_timestamp TIMESTAMP WITH TIME ZONE,
    last_kill_message TEXT,
    last_kill_timestamp TIMESTAMP WITH TIME ZONE,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"lastdeaths" table:
```sql
CREATE TABLE lastdeaths (
    player_name TEXT NOT NULL,
    first_death_message TEXT,
    first_death_timestamp TIMESTAMP WITH TIME ZONE,
    last_death_message TEXT,
    last_death_timestamp TIMESTAMP WITH TIME ZONE,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"joindate" table:
```sql
CREATE TABLE joindate (
    player_name TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"messages" table:
```sql
CREATE TABLE messages (
    player_name TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"aboutuser" table:
```sql
CREATE TABLE aboutuser (
    player_name TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"joinmsgs" table:
```sql
CREATE TABLE joinmsgs (
    creator TEXT NOT NULL,
    player_name TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"leavemsgs" table:
```sql
CREATE TABLE leavemsgs (
    creator TEXT NOT NULL,
    player_name TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    server TEXT NOT NULL,
    UNIQUE(player_name, server)
);
```
"faqmsgs" table:
```sql
CREATE TABLE faqmsgs (
    entrynum INT NOT NULL,
    player_name TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    server TEXT NOT NULL
);

```
