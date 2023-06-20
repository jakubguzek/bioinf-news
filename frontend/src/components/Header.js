import React from "react";
import {Routes, Route, useNavigate} from 'react-router-dom';
import RandomArticle from "./RandomArticle";
import KeywordSelect from "./KeywordSelect";
import SearchBar from "./SearchBar";

export default function Header() {
  const navigate = useNavigate();

  const navigateToRandom = () => {
    navigate('/random');
  };

  return (
    <div className="header">
      <h1> Recent papers in bioinformatics </h1>
      <button onClick={navigateToRandom}>Random article</button>
      <SearchBar />
      <KeywordSelect />
    </div>
  )
}
