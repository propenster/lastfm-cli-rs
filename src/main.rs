use clap::Parser;
use serde_json::Value;
use anyhow::{anyhow, Result};
use reqwest;



//Config for making our HTTP API Calls
#[derive(Debug)]
struct Config{
    api_key: String,
    username: String,
    limit: u16,
    period: String,
}

impl Config {
    fn new(api_key: String, username: String, limit: u16, period: String) -> Self{
        Config { 
            api_key, 
            username, 
            limit, 
            period 
        }
    }


    fn get_uri(&self) -> String {
        format!(
            "http://ws.audioscrobbler.com/{}/?method={}&user={}&api_key={}&format={}&period={}&limit={}",
            "2.0",
            "user.gettopartists",
            &self.username,
            &self.api_key,
            "json",
            &self.period,
            &self.limit,
        )
    }



}






#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    /// Name of the person to greet
    //name: String,
    ///Your Last.fm API Key
    #[arg(short='k', long)]
    api_key: String,

    /// Your Last.fm Username
    #[arg(short='u', long)]
    username: String,

    ///The limit of Artists returnable at once
    #[arg(short='l', long)] //this short l means nothing really, Clap will default to first letter 
    limit: u16,

    ///The lookback period - how long back should I fetch records for?
    #[arg(short, long, default_value="7day")]
    period: String

}



//separate the parsing of the results into another function
fn construct_output(config: Config, json: Value) -> Result<String>{

    let period = match config.period.as_str() {
        "overall" => "",
        "7day" => " week",
        "1month" => " month",
        "3month" => " 3 months",
        "6month" => " 6 months",
        "12month" => " year",
        _ => return Err(anyhow!("Period {} not allowed. Only allow \"overall\", \"7day\", \"1month\", \"3month\", \"6month\", or \"12month\".", config.period))
    };

    let mut f = format!(
        "♫ My Top {} played artists in the past{}:",
        config.limit.to_string(),
        period
    );

    let artists = json["topartists"]["artist"].as_array().unwrap();
    for(i, artist) in artists.iter().enumerate(){
        let ending = match i {
            x if x <= (config.limit as usize - 3) => ",",
            x if x == (config.limit as usize -2) => ", &",
            _ => "",
        };

        f = format!(
            " {} {} ({}){}",
            f,
            artist["name"].as_str().unwrap(),
            artist["playcount"].as_str().unwrap(),
            ending
        );
    }

    f = format!("{}. Via #LastFM ♫", f);
    Ok(f.to_string())





}






fn main() -> Result<()> {
    let args = Args::parse();
    //println!("Hello {}!", args.name);
    println!("Hello, world!");

    let config = Config::new(
        args.api_key,
        args.username,
        args.limit,
        args.period,
    );

    //make the API call using reqwest, pipe the response to a SERDE:: Value which is like to JSONObject
    let reqsponse: Result<_, reqwest::Error> = reqwest::blocking::get(config.get_uri())?.json::<Value>();

    if let Ok(j) = reqsponse {
        let artists = j["topartists"]["artist"].as_array().unwrap();
        //loop through response JArray... and console log them
        for a in artists.iter(){
            println!(
                "{} ({})",
                a["name"].as_str().unwrap(),
                a["playcount"].as_str().unwrap(), // this is just like JSONObject.getString("playcount") in my JAVA
            );
        }
    }else {
        return Err(anyhow!("Could not convert response to JSON"))
    }

    Ok(())

}
