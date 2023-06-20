import React from "react"
import SearchBar from "./SearchBar"
import KeywordSelect from "./KeywordSelect"

import {Routes, Route, useNavigate} from 'react-router-dom';

export default function FilterBar() {
  const navigate = useNavigate();

  const navigateToRandom = () => {
    navigate('/random');
  };

  return (
    <div className="filter-bar">
      <button onClick={navigateToRandom}>Random article</button>
      <SearchBar />
      <KeywordSelect />
    </div>
  )
}
