import React from "react";

import AsyncSelect from "react-select/async";

export default function SearchBar() {
  const [query, setQuery] = React.useState();

  async function loadOptions(value) {
    const url = (value === null) ?
      "http://127.0.0.1:8080/articles" :
      `http://127.0.0.1:8080/articles?query=${value}`
    return fetch(url)
      .then(response => response.json())
      .then(data => data.map(k => ({ value: k.title, label: k.title })));
  }

  function debugSelect(opt) {
    setQuery(opt)
    console.log(setQuery)
  }

  return (
    <div className="select-box">
      <AsyncSelect
        loadOptions={loadOptions}
        closeMenuOnSelect={false}
        isClearable
        onChange={opt => debugSelect(opt)}
        placeholder="Title..."
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
