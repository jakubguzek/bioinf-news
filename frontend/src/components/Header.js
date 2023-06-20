import React from "react";
import SearchBar from "./SearchBar";
import {Routes, Route, useNavigate} from 'react-router-dom';
import RandomArticle from "./RandomArticle";

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
    </div>
  )
}
