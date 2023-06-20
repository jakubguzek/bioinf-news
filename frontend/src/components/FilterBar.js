import React from "react"
import SearchBar from "./SearchBar"
import KeywordSelect from "./KeywordSelect"
import RandomArticleButton from "./RandomArticleButton";

export default function FilterBar(props) {

  return (
    <div className="filter-bar">
      <RandomArticleButton/>
      <SearchBar setTitle={props.setTitle}/>
      <KeywordSelect setKeywords={props.setKeywords} />
    </div>
  )
}
