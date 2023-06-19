import React from "react";
import Article from "./Article"

export default function ArticleList(props) {
  return (
    <div className="article-list">
      {props.items.map((item, index) => (<Article key={item.doi} item={item} />))}
    </div>
  );
}
