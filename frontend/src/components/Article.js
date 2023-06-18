import React from "react"

function Article(props) {
  const {item, index} = props;
  
  const [article, setArticle] = React.useState({});
  const [visible, setVisible] = React.useState(false);

  return (
      <div>
        <button className="article-collapsed-button" onClick={console.log("Click")}>
          <div className="article-collapsed-entry" key={item.doi}>
            <span className="index">{index + 1}.</span>
            <span className="title">{item.title}</span>
            <span className="pub-date">{item.publication_date}</span>
            <span className="source">{item.source}</span>
          </div>
          {visible[index] && <div className="article-entry">
            <hr/>
            <p className="authors"><b>Authors:</b> {article[index].authors}</p>
            <p className="abstract"> <b>Abstract:</b> {article[index].article_abstract}</p>
            <p className="keywords"> <b>Keywords::</b> {article[index].key_words}</p>
          </div>}
        </button>
      </div>
  )
}
