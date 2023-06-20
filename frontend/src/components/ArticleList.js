import React from "react";
import Article from "./Article"
import FilterBar from "./FilterBar";

export default function ArticleList(props) {
  const [keywords, setKeywords] = React.useState([]);
  const [title, setTitle] = React.useState(null);

  function createUrl(url, keywords, title) {
    let newUrl = `${url}?`;
    if (title !== null) {
      newUrl += `query=${title}`
    }
    if (keywords.length > 0) {
      newUrl += `&key_words=${keywords.join()}`;
    }
    return newUrl.replaceAll(" ", "%20")
  }

  async function updateArticles(keywords, title) {
    const url = "http://127.0.0.1:8080/articles"
    const newUrl = createUrl(url, keywords, title)
    console.log(newUrl)
    const data = await fetch(newUrl)
      .then(response => {
        if (response.ok) {
          return response.json()
        }
      })
    props.updateItems(data)
    console.log(data)
  }

  React.useEffect(() => {
    updateArticles(keywords, title)
  }, [keywords, title])

  return (
    <div className="article-list-container">
      <FilterBar setKeywords={setKeywords} setTitle={setTitle} />
      <div className="article-list">
        {props.items ?
          props.items.map((item) => (<Article key={item.doi} item={item} />)) :
          <h5>No results were found</h5>}
      </div>
    </div>
  );
}
