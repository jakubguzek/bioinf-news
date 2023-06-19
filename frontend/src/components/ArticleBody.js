import React from "react"
import AuthorList from "./AuthorList"
import KeywordList from "./KeywordList"

export default function ArticleBody({ article }) {
  return (
    <div className="article-details">
      <h3 className="full-title">{article.title}</h3>
      <p className="doi"><b>DOI: </b> 
        <a href={`https://${article.doi}`.replace("doi:", "doi.org//")}>{article.doi}</a>
      </p>
      <p className="authors"><b>Authors: </b><AuthorList authors={article.authors} /></p>
      <p className="abstract"> <b>Abstract:</b> {article.article_abstract}</p>
      <p className="keywords"><b>Keywords::</b> <KeywordList keywords={article.key_words} /></p>
    </div >
  )
}
