import React from "react";

export default function AuthorsList(props) {
  const { authors } = props;

  return (
    <ul className="authors-list">
      {authors.map(a => <li key={a}>{a}</li>)}
    </ul>
  )
}
