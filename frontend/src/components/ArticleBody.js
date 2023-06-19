import React from "react"
import AuthorList from "./AuthorList"
import KeywordList from "./KeywordList"

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import { faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons"

export default function ArticleBody({ article }) {
  return (
    <div className="article-details">
      <h3 className="full-title">{article.title}</h3>
      <p className="doi"><b>DOI: </b>
        <a href={`https://${article.doi}`.replace("doi:", "doi.org//")} 
          target="_blank" title="open in new tab" rel="noreferrer">
          {article.doi}
        </a>
        <sup><FontAwesomeIcon icon={faArrowUpRightFromSquare} size="2xs" /></sup>
      </p>
      <p className="authors"><b>Authors: </b><AuthorList authors={article.authors} /></p>
      {article.article_abstract && <p className="abstract"> <b>Abstract:</b> {article.article_abstract}</p>}
      <p className="keywords"><b>Keywords::</b> <KeywordList keywords={article.key_words} /></p>
    </div >
  )
}
