import React from "react"
import ArticleHeading from "./ArticleHeading";
import ArticleBody from "./ArticleBody";

export default function Article(props) {
  const { item } = props;
  const [isVisible, setIsVisible] = React.useState(false);

  function changeVisibility() {
    setIsVisible(prevIsVisible => !prevIsVisible)
  }

  return (
    <div className="article-entry">
      <button className="article-collapsed-button" onClick={changeVisibility}>
        <ArticleHeading item={item} isVisible={isVisible} />
        <hr />
      </button>
      {isVisible && <ArticleBody key={item.doi} article={item} />}
    </div>
  )
}
