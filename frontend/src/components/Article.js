import React from "react"
import ArticleHeading from "./ArticleHeading";
import AuthorList from "./AuthorList";
import KeywordList from "./KeywordList";

export default function Article(props) {
  const { item } = props;
  const [isVisible, setIsVisible] = React.useState(false);

  function changeVisibility() {
    setIsVisible(prevIsVisible => !prevIsVisible)
  }

  return (
    <div>
      <button className="article-collapsed-button" onClick={changeVisibility}>
        <ArticleHeading item={item} isVisible={isVisible} />
          <hr />
        {isVisible && <div className="article-entry">
          <p className="authors"><b>Authors: </b><AuthorList authors={item.authors} /></p>
          <p className="abstract"> <b>Abstract:</b> {item.article_abstract}</p>
          <p className="keywords"><b>Keywords::</b> <KeywordList keywords={item.key_words} /></p>
        </div>}
      </button>
    </div>
  )
}
