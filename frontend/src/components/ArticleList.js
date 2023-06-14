import React from "react";

export default function ArticleList(props) {
  const [articles, setArticles] = React.useState(new Array(props.items.length).fill(""));
  const [visible, setVisible] = React.useState(new Array(props.items.length).fill(false))

  function showDetails(id, index) {
    if (!articles[index]) {
      get_article(id, index)
    }
    const v = structuredClone(visible);
    if (v[index]) {
      v[index] = false;
    } else {
      v[index] = true;
    }
    setVisible(v);
  }

  async function get_article(id, index) {
    const article = await fetch(`http://127.0.0.1:8080/articles/${id}`)
      .then(res => res.json());
    const updatedArticles = structuredClone(articles);
    updatedArticles[index] = article;
    setArticles(updatedArticles);
    console.log(updatedArticles);
  }

  function formatTextFromArray(arr) {
    try {
      return arr.map(item => `${item}, `)
    } catch(err) {
      console.log(err)
    }
  }

  return (
    <div className="article-list">
      <h1> Recent papers in bioinformatics </h1>  {
        props.items.map((item, index) => (
          <div>
            <button className="article-collapsed-button" onClick={() => showDetails(item._id, index)}>
              <div className="article-collapsed-entry" key={item.doi}>
                <span className="index">{index + 1}.</span>
                <span className="title">{item.title}</span>
                <span className="pub-date">{item.publication_date}</span>
                <span className="source">{item.source}</span>
              </div>
              {visible[index] && <div className="article-entry">
                <hr/>
                <p className="authors"><b>Authors:</b> {formatTextFromArray(articles[index].authors)}</p>
                <p className="abstract"> <b>Abstract:</b> {articles[index].article_abstract}</p>
                <p className="keywords"> <b>Keywords::</b> {formatTextFromArray(articles[index].key_words)}</p>
              </div>}
            </button>
          </div>
        ))
      }
    </div>
  );
}
