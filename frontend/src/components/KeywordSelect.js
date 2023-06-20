import React from "react";

import AsyncSelect from "react-select/async";

export default function KeywordSelect() {
  const [chosenKeyowrds, setChosenKeywords] = React.useState([]);

  async function loadOptions() {
    return fetch("http://127.0.0.1:8080/keywords")
      .then(response => response.json())
      .then(data => data.map(k => ({ value: k, label: k })));
  }

  function debugSelect(opt) {
    setChosenKeywords(opt)
    console.log(chosenKeyowrds)
  }

  return (
    <div className="search-bar-box">
      <AsyncSelect
        loadOptions={loadOptions}
        closeMenuOnSelect={false}
        isMulti
        onChange={opt => debugSelect(opt)}
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
