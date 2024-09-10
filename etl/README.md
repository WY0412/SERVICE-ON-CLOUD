# ETL pipeline
1. Upload ETL_v4.html to databrick
2. join two table by `CREATE TABLE combinedScores_and_descriptions AS SELECT * FROM combinedScores LEFT Join screen_name_and_description_latest on combinedScores.to_user_id = screen_name`
3. create necessary index on `from_user_id`