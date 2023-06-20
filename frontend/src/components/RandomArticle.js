import React from "react"
import ArticleBody from "./ArticleBody"

export default function RandomArticle() {
  const [loading, setLoading] = React.useState(true);
  const [article, setArticle] = React.useState(null);

  function fetchArticle() {
    fetch("http://127.0.0.1:8080/random-article")
      .then(response => response.json())
      .then(data => {
        setArticle(data)
        setLoading(false)
        console.log(data)
      })
  }

  React.useEffect(() => {
    fetchArticle()
  }, [])

  return (
    <div>
      {loading ? <p>Loading the data...</p> : <ArticleBody article={article} />}
    </div >
  );
}
