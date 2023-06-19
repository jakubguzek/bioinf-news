import React from "react";

import Select from "react-select";

export default function SearchBar() {
  const availibleKeywords = [
    { label: "Bioinformatics", value: "Bioinformatics" },
    { label: "Biotechnology", value: "Biotechnology" },
    { label: "Mathematics", value: "Mathematics" },
    { label: "Cell Biology", value: "Cell Biology" }
  ];

  const [chosenKeyowrds, setChosenKeywords] = React.useState([]);

  return (
    <div className="search-bar-box">
      <Select
        options = {availibleKeywords}
        isMulti
        onChange={opt => setChosenKeywords(opt)}
        placeholder="Keyword"
        styles={{
            placeholder: (baseStyles, state) => ({
              ...baseStyles,
            display: 'flex',
            }),
          }}
      />
    </div>
  )
}
