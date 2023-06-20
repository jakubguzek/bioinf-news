import React from "react";
import Article from "./Article"
import FilterBar from "./FilterBar";

export default function ArticleList(props) {
  const [items, setItems] = React.useState([]);

  const [keywords, setKeywords] = React.useState([]);
  const [title, setTitle] = React.useState(null);

  const [showLoadMoreButton, setShowLoadMoreButton] = React.useState(false);

  function createUrl(url, keywords, title, id, publication_date) {
    const searchParams = new URLSearchParams();
    if (title !== null) {
      searchParams.set("query",title)
    }
    if (id !== undefined) {
      searchParams.set("_id",id)
    }
    if (publication_date !== undefined) {
      searchParams.set("publication_date",publication_date)
    }
    if (keywords.length > 0) {
      searchParams.set("key_words",keywords.join())
    }
    return `${url}?${searchParams.toString()}`
  }

  async function updateArticles(keywords, title) {
    const url = "http://127.0.0.1:8080/articles"
    const newUrl = createUrl(url, keywords, title)
    const data = await fetch(newUrl)
      .then(response => {
        if (response.ok) {
          return response.json()
        }
      })
    setItems(data)

    if (data){
      setShowLoadMoreButton(true)
    }

    props.onArticlesLoaded()
  }

  React.useEffect(() => {
    updateArticles(keywords, title)
  }, [keywords, title])

  const loadMoreArticles = async () => {
    if (!items || items.length == 0) {
      return
    }
    const {_id, publication_date} = items.at(-1)
    const url = "http://127.0.0.1:8080/articles"
    const newUrl = createUrl(url, keywords, title, _id.$oid, publication_date)
    const data = await fetch(newUrl)
      .then(response => {
        if (response.ok) {
          return response.json()
        }
      })

    if (!data){
      setShowLoadMoreButton(false)
      return
    }

    setItems(old => [...old, ...data])
  }

  return (
    <div className="article-list-container">
      <FilterBar setKeywords={setKeywords} setTitle={setTitle} />
      <div className="article-list">
        {items ?
          <>
            {items.map((item) => (<Article key={item.doi} item={item} />))}
            {showLoadMoreButton &&
              <div className="load-more-button-container">
                <button className="button" onClick={loadMoreArticles}>Load more</button>
              </div>
            }
          </> :
          <h5>No results were found</h5>}
      </div>
    </div>
  );
}
