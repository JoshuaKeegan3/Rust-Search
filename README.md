# Rust Search Engine.
Searches HTML files locally using [TF/IDF](https://en.wikipedia.org/wiki/Tf–idf). MVP. 

# TODO
- Performace improvement: the tfidf is basically using a tripple indented for loop with .filter()
- Clickable links in the browser.
- Use a clicked frequence algorithm like the one used in zoxide. (currently not using the log operation of tfidf as it is not a required operation my need to add back)
- Add stemming to the words (no clue how to to this. Maybe ed ing es and remove them but this doesn't work because of words ending e.) 
- Use Postgres + Docker for fun and no local file
