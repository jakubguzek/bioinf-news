import React from "react";
import Article from "./Article"

export default function ArticleList(props) {
  return (
    <div className="article-list">
      <h1> Recent papers in bioinformatics </h1>  {
        props.items.map((item, index) => (
          <Article item={item} index={index}/>
        ))
      }
    </div>
  );
}
