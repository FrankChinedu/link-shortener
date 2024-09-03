CREATE TABLE IF NOT EXISTS settings 
(
  id                        text DEFAULT 'DEFAULT_SETTINGS' NOT NULL PRIMARY KEY,
  encrypted_global_api_key  text                            NOT NULL

);

INSERT INTO settings (encrypted_global_api_key)
VALUES ('3b93ed825b4fd307cb7794b10fff1caf739fb129b321694e33783f408b25377a');
