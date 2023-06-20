import React from "react";
import {Link} from "react-router-dom";

export default function RandomArticleButton() {
    return (<Link to="/random" className="random-article-button-link"><div className="button">Random article</div></Link>
    )
  }