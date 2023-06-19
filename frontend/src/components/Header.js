import React from "react";
import SearchBar from "./SearchBar";

export default function Header() {
  return (
    <div className="header">
      <h1> Recent papers in bioinformatics </h1>  
      <SearchBar />
    </div>
  )
}
