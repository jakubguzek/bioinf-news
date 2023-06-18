import React from "react"

export default function Article(props) {
  const { item, index } = props;
  const [isVisible, setIsVisible] = React.useState(false);

  function changeVisibility() {
    setIsVisible(prevIsVisible => !prevIsVisible)
  }

  return (
    <div>
      <button className="article-collapsed-button" onClick={changeVisibility}>
        <div className="article-collapsed-entry" key={item.doi}>
          <span className="index">{index + 1}.</span>
          <span className="title">{item.title}</span>
          <span className="pub-date">{item.publication_date}</span>
          <span className="source">{item.source}</span>
        </div>
        {isVisible && <div className="article-entry">
          <hr />
          <p className="authors"><b>Authors:</b> {item.authors}</p>
          <p className="abstract"> <b>Abstract:</b> {item.article_abstract}</p>
          <p className="keywords"> <b>Keywords::</b> {item.key_words}</p>
        </div>}
      </button>
    </div>
  )
}
