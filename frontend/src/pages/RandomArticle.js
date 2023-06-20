import React from "react"
import ArticleBody from "../components/ArticleBody"

import SquareLoader from "react-spinners/SquareLoader"

export default function RandomArticle() {
  const [loading, setLoading] = React.useState(true);
  const [article, setArticle] = React.useState(null);

  async function fetchArticle() {
    const data = await fetch("http://127.0.0.1:8080/random-article")
      .then(response => response.json());
    setArticle(data);
    setLoading(false);
  }

  React.useEffect(() => {
    fetchArticle()
  }, [])

  return (
    <div>
      {loading ?
        <div className="loader-container"> <SquareLoader color={"DarkSalmon"} /> </div> :
        <ArticleBody article={article} />}
    </div>
  );
}
