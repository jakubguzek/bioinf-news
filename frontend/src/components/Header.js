import React from "react";
import KeywordSelect from "./KeywordSelect";

export default function Header() {
  return (
    <div className="header">
      <h1> Recent papers in bioinformatics </h1>  
      <KeywordSelect />
    </div>
  )
}
