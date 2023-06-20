# TODO

The number located second to the ckeckbox is the priority of the task: 

 - 0 - critical
 - 1 - important
 - 2 - kinda important 
 - 3 - nice to have
 - 4 - optional

## Frontend
- [x] 1 Make `ArticleList.js` component look pretty [Jakub]
- [x] 0 Add `ArticleList.js` links to article in detailed article view [Jakub]
- [x] 4 Possibly refactor the part that displays the article entries in `ArticleList.js` to separate component `Article.js` [Jakub]
- [ ] 0 Implement pagination either in a single-site manner or as an href
- [ ] 2 Make a nice header for the main site
- [x] 3 Clean-up code [Jakub]
- [x] 3 Comment the code [Jakub]
- [x] 2 Add button for random article [Paulina]
- [x] 0 Add filtering by keyword to front [Jakub]
- [x] 0 Add searching to front [Jakub]

## Backend
- [X] 2 Create a `random_article` endpoint [Mateusz]
- [X] 3 Maybe return a `Article` json instead of `ArticleShort` json in `get_articles_endpoint` [Mateusz]
- [ ] 4 Create an additional service that uses PubMed (entrez [E-utilities](https://pubmed.ncbi.nlm.nih.gov/download/)) to get article metadata
- [x] 1 Implement simple searching and filtering [Mateusz]
