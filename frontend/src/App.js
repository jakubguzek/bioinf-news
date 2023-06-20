import React from "react";
import './style.css';
import ArticleList from "./components/ArticleList";
import Footer from "./components/Footer"
import Header from "./components/Header";

import SquareLoader from "react-spinners/SquareLoader"

export default function App() {
  const [items, setItems] = React.useState([]);
  const [loading, setLoading] = React.useState(true);


  async function fetchArticles() {
    const data = await fetch("http://127.0.0.1:8080/articles")
      .then((res) => res.json());
    setItems(data);
    setLoading(false);
  }

  React.useEffect(() => {
    fetchArticles()
  }, [])

  return (
    <div className="container">
      {loading ?
        <div className="loader-container">
          <SquareLoader color={"DarkSalmon"} />
        </div> :
        <div>
          <Header />
          <ArticleList items={items} />
          <Footer />
        </div>}
    </div>
  );
}

