import React from "react";

export default function ArticleList(props) {
  return (
    <div className="article-list">
      <h1> Recent papers in bioinformatics </h1>  {
        props.items.map((item) => (
          <div>
            <button className="article-collapsed-button" onClick={console.log()}>
              <div className="article-collapsed-entry" key={item.doi}>
                <span className="title">{item.title}</span>
                <span className="pub-date">{item.publication_date}</span>
                <span className="source">{item.source}</span>
              </div>
            </button>
          </div>
        ))
      }
    </div>
  )
}
