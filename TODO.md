# TODO

The number located second to the ckeckbox is the priority of the task: 

 - 0 - critical
 - 1 - important
 - 2 - kinda important 
 - 3 - nice to have
 - 4 - optional

## Frontend
- [ ] 1 Make `ArticleList.js` component look pretty [Jakub]
- [ ] 0 Add `ArticleList.js` links to article in detailed article view [Jakub]
- [ ] 4 Possibly refactor the part that displays the article entries in `ArticleList.js` to separate component `Article.js` [Jakub]
- [ ] 0 Implement pagination either in a single-site manner or as an href
- [X] 1 Implement simple searching and filtering [Mateusz]
- [ ] 2 Make a nice header for the main site
- [ ] 3 Clean-up and comment the code

## Backend
- [X] 2 Create a `random_article` endpoint [Mateusz]
- [X] 3 Maybe return a `Article` json instead of `ArticleShort` json in `get_articles_endpoint` [Mateusz]
- [ ] 4 Create an additional service that uses PubMed (entrez [E-utilities](https://pubmed.ncbi.nlm.nih.gov/download/)) to get article metadata

