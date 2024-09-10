
use std::error::Error;
use sqlx::Row;
use sqlx::FromRow;
use sqlx::mysql::{MySqlPool};
use crate::kmp::KMP;
extern crate chrono;

const MALFORMED_RESPONSE: &str = "CCprojectFor3,851725245278\nINVALID";
const SUCCESS_RESPONSE: &str = "CCprojectFor3,851725245278";

#[derive(Debug)]
struct Record {
    id: i64,                // User ID
    score: f64,             // Total score, used for sorting
    screen_name: String,    // User's screen name
    description: String,    // User's description
    latest_tweet_text: String,           // Latest tweet text
}

#[derive(Debug)]
#[derive(FromRow)]
struct ResRow {
    to_user_id: i64,
    interaction_score: f64,
    hashtag_score: f64,
    reply_aggregated_texts: Option<String>,
    retweet_aggregated_texts: Option<String>,
    reply_aggregated_hashtags: Option<String>,
    retweet_aggregated_hashtags: Option<String>,
    latest_retweet_text: Option<String>,
    latest_reply_text: Option<String>,
    latest_retweet_text_timestamp: Option<String>,
    latest_reply_text_timestamp: Option<String>,
    latest_screen_name: Option<String>,
    latest_description: Option<String>,
}

fn count_overlapping_substrings(text: &str, pattern: &str) -> i32 {
    let kmp = KMP::new(pattern);
    return kmp.count_overlap(text);
}


pub async fn calculate_ranking_score(pool: &MySqlPool, from_user_id: i64, interaction_type: &str, key_phrase: &str, hashtag: &str) -> Result<String, Box<dyn Error>> {
    println!("start");
    if interaction_type != "reply" && interaction_type != "retweet" && interaction_type != "both"{
        return Ok(MALFORMED_RESPONSE.to_string());
    }

    let lowercase_phrase = key_phrase.to_lowercase();

    let rows = sqlx::query_as::<_, ResRow>( 
    "SELECT to_user_id, interaction_score, hashtag_score, reply_aggregated_texts, 
        retweet_aggregated_texts, reply_aggregated_hashtags, retweet_aggregated_hashtags, latest_retweet_text, 
        latest_reply_text, 
        CAST(latest_retweet_text_timestamp AS CHAR) AS latest_retweet_text_timestamp,
        CAST(latest_reply_text_timestamp AS CHAR) AS latest_reply_text_timestamp,
        latest_screen_name, latest_description
        FROM combinedScores_and_descriptions
        WHERE from_user_id = ?").bind(from_user_id).fetch_all(pool).await.unwrap();

    let mut records: Vec<Record> = Vec::new();

    if rows.is_empty() {
        return Ok(MALFORMED_RESPONSE.to_string());
    }

    for row in rows {
        //TODO, split by \n and calculate separately, but might not be necessary
        let mut number_of_matches = 0.0;
        if interaction_type == "reply" {
            if row.reply_aggregated_texts.is_none() && row.reply_aggregated_hashtags.is_none() {
                continue;
            }
            number_of_matches += count_overlapping_substrings(&row.reply_aggregated_texts.unwrap_or("".to_string()), &lowercase_phrase) as f64;
            number_of_matches += count_overlapping_substrings(&row.reply_aggregated_hashtags.unwrap_or("".to_string()), hashtag) as f64;
        } else if interaction_type == "retweet" {
            if row.retweet_aggregated_texts.is_none() && row.retweet_aggregated_hashtags.is_none() {
                continue;
            }
            number_of_matches += count_overlapping_substrings(&row.retweet_aggregated_texts.unwrap_or("".to_string()), &lowercase_phrase) as f64;
            number_of_matches += count_overlapping_substrings(&row.retweet_aggregated_hashtags.unwrap_or("".to_string()), hashtag) as f64;
        } else {
            // might not need to check, contact tweet should have some interactions?
            number_of_matches += count_overlapping_substrings(&row.reply_aggregated_texts.unwrap_or("".to_string()), &lowercase_phrase) as f64;
            number_of_matches += count_overlapping_substrings(&row.reply_aggregated_hashtags.unwrap_or("".to_string()), hashtag) as f64;
            number_of_matches += count_overlapping_substrings(&row.retweet_aggregated_texts.unwrap_or("".to_string()), &lowercase_phrase) as f64;
            number_of_matches += count_overlapping_substrings(&row.retweet_aggregated_hashtags.unwrap_or("".to_string()), hashtag) as f64;
        }
        let keywords_score = 1.0 + (number_of_matches as f64).ln_1p(); // Using ln(1+x) for scaling
        let total_score = row.interaction_score * row.hashtag_score * keywords_score;
        
        // let (screen_name, description) = get_latest_screen_name_and_description_by_id(pool, row.to_user_id).await?;
        let latest_tweet_text;
        if interaction_type == "reply" {
            latest_tweet_text = row.latest_reply_text.unwrap_or("".to_string());
        } else if interaction_type == "retweet" {
            latest_tweet_text = row.latest_retweet_text.unwrap_or("".to_string());
        } else { //case for both: choose the one with greater timestamp
            if row.latest_reply_text_timestamp.unwrap_or("".to_string()) > row.latest_retweet_text_timestamp.unwrap_or("".to_string()) {
                latest_tweet_text = row.latest_reply_text.unwrap_or("".to_string());
            } else {
                latest_tweet_text = row.latest_retweet_text.unwrap_or("".to_string());
            }
        }
        records.push(Record {
            id: row.to_user_id,       // Assuming this is the ID you referred to.
            score: total_score,       // The calculated total score.
            screen_name: row.latest_screen_name.unwrap_or("".to_string()),  // The screen name of the user.
            description: row.latest_description.unwrap_or("".to_string()),  // The description of the user.
            latest_tweet_text: latest_tweet_text, // The latest tweet text.
        });
    }

    records.sort_by(|a, b| {
        // First, try to compare the scores.
        let score_order = b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal);
    
        // If scores are equal (Ordering::Equal), then compare by user_id.
        if score_order == std::cmp::Ordering::Equal {
            // Note the order: b compared to a for descending order.
            b.id.cmp(&a.id)
        } else {
            score_order
        }
    });

    let mut formatted_strings = SUCCESS_RESPONSE.to_string();

    for record in records {
        // Format each record into the desired string format
        let formatted_string = format!(
            "\n{}\t{}\t{}\t{}",
            record.id, record.screen_name, record.description, record.latest_tweet_text
        );
        formatted_strings.push_str(&formatted_string);
    }

    Ok(formatted_strings)
}

async fn get_latest_screen_name_and_description_by_id(pool: &MySqlPool, user_id: i64) -> Result<(String, String), Box<dyn Error>> {
    // println!("user_id {}",user_id);
    let rows= sqlx::query("SELECT latest_screen_name, latest_description
                 FROM screen_name_and_description_latest
                 WHERE user_id = ?").bind(user_id).fetch_all(pool).await.unwrap();


    let mut screen_name = String::new();
    let mut description = String::new();
    for row in rows {
        screen_name = row.try_get("latest_screen_name").unwrap_or("".to_string());
        // println!("{}",screen_name);
        description = row.try_get("latest_description").unwrap_or("".to_string());
        // println!("{}",description);
    }
    Ok((screen_name, description))
}


#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use sqlx::{Connection, ConnectOptions};
    use sqlx::mysql::{MySqlConnectOptions, MySqlConnection, MySqlPool, MySqlSslMode, MySql};
    use tokio;

    #[tokio::test]
    async fn test_run() {

        let password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set");
        let host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set");
        let user = env::var("MYSQL_USER").expect("MYSQL_USER is not set");
        let database = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE is not set");

        let opt = MySqlConnectOptions::new().host(&*host).username(&*user).password(&*password).database(&*database);
        let pool: MySqlPool = MySqlPool::connect_with(opt).await.unwrap();
        //let mut conn: PoolConnection<MySql> = pool.acquire().await.expect("Failed to acquire a connection.");

        let res = calculate_ranking_score(&pool, 5669252, "retweet", "uwcomm", "uwcomm");
        print!("{}", res.await.unwrap());
    }

}