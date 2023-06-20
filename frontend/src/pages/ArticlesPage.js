import React from "react";
import ArticleList from "../components/ArticleList";
import Header from "../components/Header";
import SquareLoader from "react-spinners/SquareLoader"

export default function ArticlesPage() {

  const [loading, setLoading] = React.useState(true);

  const handleArticlesLoaded = () => {
    setLoading(false)
  }

  return (
    <div className="container">
          <Header />
          <ArticleList onArticlesLoaded={handleArticlesLoaded}/>
          {loading && <div className="loader-container"><SquareLoader color={"DarkSalmon"} /></div>}
    </div>
  );
}
