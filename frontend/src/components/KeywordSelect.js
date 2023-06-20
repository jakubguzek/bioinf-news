import React from "react";

import AsyncSelect from "react-select/async";

export default function KeywordSelect(props) {
  async function loadOptions() {
    return fetch("http://127.0.0.1:8080/keywords")
      .then(response => response.json())
      .then(data => data.map(k => ({ value: k, label: k })));
  }

  function keywordSelect(opt) {
    const keywords = opt.map(o => o.value)
    props.setKeywords(keywords)
  }

  return (
    <div className="select-box">
      <AsyncSelect
        loadOptions={loadOptions}
        closeMenuOnSelect={false}
        isMulti
        onChange={opt => keywordSelect(opt)}
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
