import React from "react";

export default function KeywordList(props) {
  const { keywords } = props;

  return (
    <ul className="keyword-list">
      {keywords.map(k => <li key={k}>{k}</li>)}
    </ul>
  )
}
