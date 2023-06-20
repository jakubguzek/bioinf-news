import React from "react";
import Article from "./Article"
import FilterBar from "./FilterBar";

export default function ArticleList(props) {
  return (
    <div>
    <FilterBar />
    <div className="article-list">
      {props.items.map((item) => (<Article key={item.doi} item={item} />))}
    </div>
    </div>
  );
}
