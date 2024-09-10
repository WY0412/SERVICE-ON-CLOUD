import org.apache.spark.sql.SparkSession
import org.apache.spark.sql.functions._
import org.apache.spark.sql.types.{StructType, IntegerType, StringType, DoubleType}
// import org.apache.spark.sql.functions.{lit, size, col, collect_list, explode, sum }
import java.util.Properties


object Twitter {

    def main(args: Array[String]): Unit = {
        val spark = SparkSession.builder.appName("Filter Tweets").getOrCreate()       
        import spark.implicits._

        val small_twitter = spark.read.option("driver", "org.postgresql.Driver").json("/small_twitter.json")
        val df = spark.read.json("wasb://datasets@clouddeveloper.blob.core.windows.net/twitter-dataset/part-r-00000.gz")

        val tweets = df
        tweets.printSchema


        val validTweets = tweets.filter(
            ($"id".isNotNull || $"id_str".isNotNull) &&
            ($"user.id".isNotNull || $"user.id_str".isNotNull) &&
            $"created_at".isNotNull &&
            $"text".isNotNull && $"text" =!= "" &&
            $"entities.hashtags".isNotNull && size($"entities.hashtags") =!= 0 &&
            array_contains(array(lit("ar"), lit("en"), lit("fr"), lit("in"), lit("pt"), lit("es"), lit("tr"), lit("ja")), $"lang")
        )

        val updatedTweets = validTweets.withColumn("user.id",
            when($"user.id".isNull && $"user.id_str".isNotNull, $"user.id_str".cast("long"))
            .otherwise($"user.id")
        )

        val uniqueTweets = updatedTweets.dropDuplicates("id_str").cache

        val flattenListsUDF = udf((lists: Seq[Seq[String]]) => lists.flatten)

    // Now, when you aggregate, you can collect the flattened hashtags

        val replies = uniqueTweets
            .where(col("in_reply_to_user_id").isNotNull)
            .withColumn("from_user_id", col("user.id"))
            .withColumn("created_at_corrected", expr("substring(created_at, 5, length(created_at))"))
            .withColumn("created_at_ts", unix_timestamp(col("created_at_corrected"), "MMM dd HH:mm:ss Z yyyy").cast("timestamp"))
            .groupBy("from_user_id", "in_reply_to_user_id")
            .agg(
                count("*").alias("reply_count"),
                substring(concat_ws("\n", collect_list("text")), 1, 65535).alias("reply_aggregated_texts"),
                concat_ws("\n", flattenListsUDF(collect_list("entities.hashtags.text"))).alias("reply_aggregated_hashtags"),
                max(struct(col("created_at_ts"), col("id"), col("text"))).alias("latest") // Including tweet_id for tie-breaking
            )
            .withColumn("latest_tweet_text", col("latest.text")) // Extracting the text from the latest struct
            .withColumnRenamed("in_reply_to_user_id", "to_user_id")

        // Now, when you aggregate, you collect the flattened hashtags and concatenate texts
        val retweets = (uniqueTweets
        .where(col("retweeted_status").isNotNull)
        .withColumn("from_user_id", col("user.id"))
        .withColumn("to_user_id", col("retweeted_status.user.id"))
        .groupBy("from_user_id", "to_user_id")
        .agg(
            count("*").alias("retweet_count"),
            concat_ws("\n", collect_list("text")).alias("retweet_aggregated_texts"),
            concat_ws("\n", flattenListsUDF(collect_list("entities.hashtags.text"))).alias("retweet_aggregated_hashtags")
        ))

        // Join replies and retweets DataFrames to combine interaction counts for each user pair
        val combinedInteractions = (replies
        .join(retweets, Seq("from_user_id", "to_user_id"), "outer")
        .na.fill(0)) // Replace null counts with 0

        // Calculate the interaction score
        val interactionScores = (combinedInteractions
        .withColumn("interaction_score", log(lit(1) + (col("reply_count") * 2) + col("retweet_count"))))


        // Use Scala's Source.fromFile to read the local file
        val excludedHashtagsPath = "/popular_hashtags.txt" // Update this to your file's path
        val excludedHashtagsDF = spark.read.text(excludedHashtagsPath).withColumnRenamed("value", "hashtag").select(lower(col("hashtag")).as("hashtag"))


        // Explode the hashtags array and normalize to lowercase, excluding undesired tags
        val explodedHashtags = (uniqueTweets
        .withColumn("user_id", col("user.id")) 
        .withColumn("hashtag_nested", explode(col("entities.hashtags")))
        .withColumn("hashtag", lower(col("hashtag_nested.text")))
        .select("user_id", "hashtag") )

        val explodedHashtagsFiltered = ( explodedHashtags
        .join(broadcast(excludedHashtagsDF), explodedHashtags("hashtag") === excludedHashtagsDF("hashtag"), "left_anti") )
        
        
        val hashtagPairs = (explodedHashtagsFiltered.as("df1")
        .join(explodedHashtagsFiltered.as("df2"), $"df1.hashtag" === $"df2.hashtag" && $"df1.user_id" =!= $"df2.user_id")
        .groupBy($"df1.user_id".as("from_user_id"), $"df2.user_id".as("to_user_id"))
        .agg(count($"df1.hashtag").as("same_tag_count")) )

        val combinedScores = (hashtagPairs
        .join(interactionScores, Seq("from_user_id", "to_user_id"), "right")
        .withColumn("hashtag_score", when($"same_tag_count" > 10, lit(1) + log(lit(1) + $"same_tag_count" - lit(10))).otherwise(lit(1)))
        .select("from_user_id", "to_user_id", "hashtag_score","interaction_score","reply_aggregated_texts","reply_aggregated_hashtags","retweet_aggregated_texts","retweet_aggregated_hashtags","latest_tweet_text"))

        combinedScores.show()

        // val jdbcPort = "3306"
        // val jdbcDatabase = "etl_db2"
        // val jdbcUsername = "etl_user"
        // val jdbcPassword = "PAYTON401gzt"   // the password you set
        // val jdbcHostname = "34.139.20.62"   // the external IP address of you MySQL DB

        // val connectionProperties = new Properties()
        // connectionProperties.put("user", jdbcUsername)
        // connectionProperties.put("password", jdbcPassword)

        // val jdbcUrl =s"jdbc:mysql://$jdbcHostname:$jdbcPort/$jdbcDatabase"
        // val tableName = "kewen"
        // combinedScores.write.jdbc(jdbcUrl, tableName, connectionProperties)


val screen_name_and_description_latest = uniqueTweets
  .withColumn("user_id", col("user.id"))
  .withColumn("created_at_corrected", expr("substring(created_at, 5, length(created_at))"))
  .withColumn("created_at_ts", unix_timestamp(col("created_at_corrected"), "MMM dd HH:mm:ss Z yyyy").cast("timestamp"))
  .groupBy("user_id") // Ensure you are grouping by necessary IDs
  .agg(
    max(struct(col("created_at_ts"), col("id"), col("user.screen_name").alias("screen_name"),
    col("user.description").alias("description"))).alias("latest")
  )
  .select(
    col("user_id"), 
    col("latest.screen_name").alias("latest_screen_name"), 
    col("latest.description").alias("description")
  )

        import java.util.Properties
import java.sql.DriverManager

// set credentials
val jdbcPort = "3306"
val jdbcDatabase = "etl_db"
val jdbcUsername = "etl_user"
val jdbcPassword = "PAYTON401gzt"   // the password you set
val jdbcHostname = "35.231.233.134"   // the external IP address of you MySQL DB

val jdbcUrl =s"jdbc:mysql://$jdbcHostname:$jdbcPort/$jdbcDatabase"

val driverClass = "com.mysql.cj.jdbc.Driver"
Class.forName(driverClass)  // check jdbc driver

// set connection properties
val connectionProperties = new Properties()
connectionProperties.put("user", s"$jdbcUsername")
connectionProperties.put("password", s"$jdbcPassword")
connectionProperties.setProperty("Driver", driverClass)
connectionProperties.setProperty("useServerPrepStmts", "false")    // take note of this configuration, and understand what it does
connectionProperties.setProperty("rewriteBatchedStatements", "true")  // take note of this configuration, and understand what it does

// first drop the table if it already exists
val connection = DriverManager.getConnection(jdbcUrl, jdbcUsername, jdbcPassword)
assert(!connection.isClosed)
val stmt = connection.createStatement()

stmt.executeUpdate("drop table if exists combinedScores")
stmt.executeUpdate("drop table if exists screen_name_and_description_latest")

// write the dataframe to a table called "biz_review"
combinedScores.write.jdbc(jdbcUrl, "combinedScores", connectionProperties)

screen_name_and_description_latest.write.jdbc(jdbcUrl, "screen_name_and_description_latest", connectionProperties)

    }

}